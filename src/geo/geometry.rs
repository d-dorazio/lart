use crate::{Geometry, Path, Polygon};

impl Geometry {
    pub const fn new() -> Self {
        Self {
            polygons: vec![],
            paths: vec![],
        }
    }

    pub fn extend(&mut self, o: &Self) {
        self.polygons.extend_from_slice(&o.polygons);
        self.paths.extend_from_slice(&o.paths);
    }
}

impl From<Polygon> for Geometry {
    fn from(p: Polygon) -> Self {
        Geometry {
            polygons: vec![p],
            paths: vec![],
        }
    }
}

impl From<Path> for Geometry {
    fn from(p: Path) -> Self {
        Geometry {
            polygons: vec![],
            paths: vec![p],
        }
    }
}
