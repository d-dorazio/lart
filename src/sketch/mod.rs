pub mod parms;
pub mod vpype;

pub use parms::*;
pub use vpype::*;

use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs,
    io::{self, BufWriter, Write},
    path::Path as FsPath,
};

pub use rand::prelude::*;
pub use rand_xoshiro::Xoshiro256StarStar;

use crate::{v, Geometry, Rect, Xform};

pub type MyRng = Xoshiro256StarStar;

pub trait Plugin: 'static {
    fn execute(&self, svg: &str);
}

pub struct Sketch {
    name: String,
    page: Page,

    seed: u64,
    rng: MyRng,

    layer_id: i32,
    layers: BTreeMap<i32, Layer>,

    plugins: Vec<Box<dyn Plugin>>,

    background: String,
}

pub struct Layer {
    geo: Geometry,
    fill: String,
    stroke: String,
    pen_width: f64, // mm
}

pub struct Page(pub f64, pub f64);

#[macro_export]
macro_rules! skv_log {
    ($command:expr, $value:expr) => {
        if std::env::var("SKV_VIEWER").is_ok() {
            println!("#SKV_VIEWER_COMMAND {}={}", $command, $value);
        }
    };
}

impl Sketch {
    pub fn new(name: &str) -> Self {
        let seed = thread_rng().gen::<u64>();

        Self {
            name: name.to_string(),
            page: Page::A4,
            seed,
            rng: MyRng::seed_from_u64(seed),
            layer_id: 1,
            layers: BTreeMap::new(),
            plugins: vec![],
            background: String::new(),
        }
    }

    pub fn with_page(mut self, page: Page) -> Self {
        self.page = page;
        self
    }

    pub fn with_name(mut self, s: &str) -> Self {
        self.name = s.to_owned();
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self.rng = MyRng::seed_from_u64(seed);
        self
    }

    pub fn plugin(mut self, plugin: impl Plugin) -> Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn with_background(mut self, color: &str) -> Self {
        self.background = color.to_owned();
        self
    }

    pub fn rng(&mut self) -> &mut MyRng {
        &mut self.rng
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn dimensions(&self) -> (f64, f64) {
        (self.page.0, self.page.1)
    }

    pub fn page_bbox(&self) -> Rect {
        let (w, h) = self.dimensions();
        Rect::with_dimensions(v(0, 0), w, h)
    }

    pub fn geometry(&mut self, g: impl Into<Geometry>) {
        let g = g.into();
        self.layer(self.layer_id).geo.extend(&g);
    }

    pub fn layer(&mut self, lid: i32) -> &mut Layer {
        self.layer_id = lid;
        self.layers.entry(self.layer_id).or_default()
    }

    pub fn fit_to_page(&mut self, margin: f64) {
        let bbox = match self.layers_bbox() {
            None => return,
            Some(b) => b,
        };

        let mut page_bbox = self.page_bbox();
        page_bbox.pad(-margin);

        let xform = Xform::rect_to_rect(&bbox, &page_bbox);
        for l in self.layers.values_mut() {
            l.geo *= &xform;
        }
    }

    fn layers_bbox(&self) -> Option<Rect> {
        let mut bbox: Option<Rect> = None;

        for l in self.layers.values() {
            if let Some(b) = l.geo.bbox() {
                bbox = match bbox {
                    Some(mut bb) => {
                        bb.union(&b);
                        Some(bb)
                    }
                    None => Some(b),
                };
            }
        }
        bbox
    }

    pub fn save(&self) -> io::Result<()> {
        let outdir = FsPath::new(&self.name);

        if !outdir.is_dir() {
            fs::create_dir(outdir)?;
        }

        let get_next_free_name = || -> io::Result<String> {
            let mut last = 0;
            for f in fs::read_dir(outdir)? {
                let f = f?.path();

                if !f.is_file() || f.extension() != Some(OsStr::new("svg")) {
                    continue;
                }

                let n = f.file_stem().and_then(|n| {
                    n.to_string_lossy()
                        .trim_start_matches(&self.name)
                        .trim_start_matches('-')
                        .parse()
                        .ok()
                });
                if let Some(n) = n {
                    last = last.max(n);
                }
            }

            Ok(format!("{}-{}.svg", self.name, last + 1))
        };

        let outpath = outdir.join(&get_next_free_name()?);
        {
            let out = fs::File::create(&outpath)?;
            let mut out = BufWriter::new(out);

            let (width, height) = self.dimensions();

            writeln!(
                out,
                r#"<?xml version="1.0" encoding="utf-8" ?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}mm" height="{h}mm">"#,
                w = width,
                h = height
            )?;

            // TODO: save parameters in the svg here?
            // the problem is that vpype and other tools throw away comments...

            if !self.background.is_empty() {
                writeln!(
                    out,
                    r#"<rect x="0" y="0" width="{}" height="{}" stroke="none" fill="{}" />"#,
                    width, height, &self.background
                )?;
            }

            for (lid, layer) in &self.layers {
                let geo = &layer.geo;

                if geo.polygons.is_empty() && geo.paths.is_empty() {
                    continue;
                }

                writeln!(
                    out,
                    r#"<g id="layer{}" fill="{}" stroke="{}" stroke-width="{}">"#,
                    lid, layer.fill, layer.stroke, layer.pen_width
                )?;

                for path in &geo.paths {
                    if path.is_empty() {
                        continue;
                    }

                    write!(out, r#"<polyline points=""#)?;
                    for p in path.points() {
                        write!(out, "{},{} ", p.x, p.y)?;
                    }
                    writeln!(out, r#""/>"#)?;
                }

                for poly in &geo.polygons {
                    write!(out, r#"<path d=""#)?;
                    for path in &poly.areas {
                        if path.is_empty() {
                            continue;
                        }
                        write!(out, r#"M{},{} "#, path.points()[0].x, path.points[0].y)?;
                        for p in path.points().iter().skip(1) {
                            write!(out, r#"L{},{} "#, p.x, p.y)?;
                        }
                        write!(out, "Z ")?;
                    }
                    writeln!(out, r#""/>"#)?;
                }

                writeln!(out, "</g>")?;
            }

            writeln!(out, r"</svg>")?;
        }

        let outpath = outpath.canonicalize()?;
        let outpath = outpath.to_str().unwrap();
        for p in &self.plugins {
            p.execute(outpath);
        }

        skv_log!("SVG", outpath);

        Ok(())
    }
}

impl Layer {
    pub fn with_fill(&mut self, fill: &str) -> &mut Self {
        self.fill = fill.to_string();
        self
    }

    pub fn with_stroke(&mut self, stroke: &str) -> &mut Self {
        self.stroke = stroke.to_string();
        self
    }

    pub fn with_pen_width(&mut self, pen_width: f64) -> &mut Self {
        self.pen_width = pen_width;
        self
    }

    pub fn geo(&self) -> &Geometry {
        &self.geo
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            geo: Geometry::new(),
            fill: "none".to_string(),
            stroke: "black".to_string(),
            pen_width: 0.2,
        }
    }
}

impl Page {
    pub const A0: Self = Self(841.0, 1189.0);
    pub const A1: Self = Self(594.0, 841.0);
    pub const A2: Self = Self(420.0, 594.0);
    pub const A3: Self = Self(297.0, 420.0);
    pub const A4: Self = Self(210.0, 297.0);
    pub const A5: Self = Self(148.0, 210.0);
    pub const A6: Self = Self(105.0, 148.0);
}

impl RngCore for Sketch {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.rng.try_fill_bytes(dest)
    }
}
