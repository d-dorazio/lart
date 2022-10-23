use crate::{Geometry, Path, Polygon};

pub trait Chaikin {
    fn chaikin(&self, ratio: f64) -> Self;
}

impl Chaikin for Path {
    fn chaikin(&self, ratio: f64) -> Self {
        chaikin_impl(self, ratio, false)
    }
}

impl Chaikin for Polygon {
    fn chaikin(&self, ratio: f64) -> Self {
        self.areas
            .iter()
            .map(|p| chaikin_impl(p, ratio, true))
            .collect()
    }
}

impl Chaikin for Geometry {
    fn chaikin(&self, ratio: f64) -> Self {
        let mut g = Geometry::new();

        g.push_polygons(self.polygons.iter().map(|p| p.chaikin(ratio)));
        g.push_paths(self.paths.iter().map(|p| p.chaikin(ratio)));

        g
    }
}

fn chaikin_impl(path: &Path, ratio: f64, closed: bool) -> Path {
    if path.len() <= 2 {
        return path.clone();
    }

    let ratio = if ratio <= 0.5 { ratio } else { 1.0 - ratio };

    let mut new = Path::with_capacity(path.len() * 2 + if closed { 2 } else { 0 });

    if !closed {
        new.push(path[0]);
    }

    for (s, e) in path.segments() {
        let d = e - s;
        new.push(s + ratio * d);
        new.push(e - ratio * d);
    }

    if !closed {
        new.push(path.last().unwrap());
    }

    new
}
