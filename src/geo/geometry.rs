use crate::{Geometry, Path, Polygon};

impl Geometry {
    pub const fn new() -> Self {
        Self {
            polygons: vec![],
            paths: vec![],
        }
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
