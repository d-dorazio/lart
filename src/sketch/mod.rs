use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs,
    io::{self, BufWriter, Write},
    path::Path as FsPath,
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

    layer: i32,
    layers: BTreeMap<i32, Geometry>,
}

pub enum Page {
    A4,
    A5,
    Custom(f64, f64),
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
            layer: 0,
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

    pub fn dimensions(&self) -> (f64, f64) {
        self.page.dimensions()
    }

    pub fn geometry(&mut self, g: impl Into<Geometry>) {
        let g = g.into();

        let layer = self.layers.entry(self.layer).or_insert_with(Geometry::new);
        layer.polygons.extend_from_slice(&g.polygons);
        layer.paths.extend_from_slice(&g.paths);
    }

    pub fn save(&self) -> io::Result<()> {
        let outdir = FsPath::new(&self.name);

        if !outdir.is_dir() {
            fs::create_dir(outdir)?;
        }

        let mut i = 0;
        for f in fs::read_dir(outdir)? {
            let f = f?.path();

            if !f.is_file() || f.extension() != Some(OsStr::new("svg")) {
                continue;
            }

            if let Some(n) = f.file_stem() {
                if let Ok(n) = n.to_string_lossy().parse() {
                    i = i.max(n);
                }
            }
        }
        i += 1;

        let out = fs::File::create(outdir.join(format!("{i}.svg")))?;
        let mut out = BufWriter::new(out);

        let (width, height) = self.page.dimensions();

        writeln!(
            out,
            r#"<?xml version="1.0" encoding="utf-8" ?>
<svg viewBox="0 0 {w} {h}" width="{w}mm" height="{h}mm">"#,
            w = width,
            h = height
        )?;

        let dump_path_points = |out: &mut BufWriter<fs::File>, path: &Path| -> io::Result<()> {
            for p in path.points() {
                write!(out, "{},{} ", p.x, p.y)?;
            }
            Ok(())
        };

        for (lid, geo) in &self.layers {
            if geo.polygons.is_empty() && geo.paths.is_empty() {
                continue;
            }

            writeln!(
                out,
                r#"<g id="layer{}" fill="none" stroke="black" stroke-width="0.2mm">"#,
                lid + 1
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

        Ok(())
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
