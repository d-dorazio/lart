pub mod parms;

pub use parms::*;

use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs,
    io::{self, BufWriter, Write},
    path::{Path as FsPath, PathBuf},
    time::SystemTime,
};

pub use rand::prelude::*;
pub use rand_xoshiro::Xoshiro256StarStar;

use crate::{Geometry, Path};

pub type MyRng = Xoshiro256StarStar;

pub struct Sketch {
    name: String,
    page: Page,

    seed: u64,
    rng: MyRng,

    layer_id: i32,
    layers: BTreeMap<i32, Layer>,
}

pub struct Layer {
    geo: Geometry,
    fill: String,
    stroke: String,
    pen_width: String,
}

pub enum Page {
    A4,
    A5,
    Custom(f64, f64),
}

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
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            name: name.to_string(),
            page: Page::A4,
            seed,
            rng: MyRng::seed_from_u64(seed),
            layer_id: 1,
            layers: BTreeMap::new(),
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

    pub fn rng(&mut self) -> &mut MyRng {
        &mut self.rng
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn dimensions(&self) -> (f64, f64) {
        self.page.dimensions()
    }

    pub fn geometry(&mut self, g: impl Into<Geometry>) {
        let g = g.into();
        self.layer(self.layer_id).geo.extend(&g);
    }

    pub fn layer(&mut self, lid: i32) -> &mut Layer {
        self.layer_id = lid;
        self.layers.entry(self.layer_id).or_default()
    }

    pub fn save(&self) -> io::Result<PathBuf> {
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

                let n = f.file_stem().and_then(|n| n.to_string_lossy().parse().ok());
                if let Some(n) = n {
                    last = last.max(n);
                }
            }

            Ok(format!("{}.svg", last + 1))
        };

        let outpath = outdir.join(&get_next_free_name()?);
        let out = fs::File::create(&outpath)?;
        let mut out = BufWriter::new(out);

        let (width, height) = self.page.dimensions();

        writeln!(
            out,
            r#"<?xml version="1.0" encoding="utf-8" ?>
<svg viewBox="0 0 {w} {h}" width="{w}mm" height="{h}mm">"#,
            w = width,
            h = height
        )?;

        // TODO: dump parameters here?

        let dump_path_points = |out: &mut BufWriter<fs::File>, path: &Path| -> io::Result<()> {
            for p in path.points() {
                write!(out, "{},{} ", p.x, p.y)?;
            }
            Ok(())
        };

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
                dump_path_points(&mut out, path)?;
                writeln!(out, r#""/>"#)?;
            }

            for poly in &geo.polygons {
                for path in &poly.areas {
                    write!(out, r#"<polygon points=""#)?;
                    dump_path_points(&mut out, path)?;
                    writeln!(out, r#""/>"#)?;
                }
            }

            writeln!(out, "</g>")?;
        }

        writeln!(out, r"</svg>")?;

        skv_log!("SVG", outpath.canonicalize()?.to_str().unwrap());

        Ok(outpath)
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

    pub fn with_pen_width(&mut self, pen_width: &str) -> &mut Self {
        self.pen_width = pen_width.to_string();
        self
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            geo: Geometry::new(),
            fill: "none".to_string(),
            stroke: "black".to_string(),
            pen_width: "0.2mm".to_string(),
        }
    }
}

impl Page {
    fn dimensions(&self) -> (f64, f64) {
        match *self {
            Page::A4 => (210.0, 297.0),
            Page::A5 => (148.0, 210.0),
            Page::Custom(w, h) => (w, h),
        }
    }
}
