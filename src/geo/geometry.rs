use crate::{Geometry, Path, Polygon};

impl Geometry {
    pub const fn new() -> Self {
        Self {
            polygons: vec![],
            paths: vec![],
        }
    }

    pub fn from_polygons(polygons: Vec<Polygon>) -> Self {
        Self {
            polygons,
            paths: vec![],
        }
    }

    pub const fn from_paths(paths: Vec<Path>) -> Self {
        Self {
            polygons: vec![],
            paths,
        }
    }

    pub fn extend(&mut self, o: &Self) {
        self.polygons.extend_from_slice(&o.polygons);
        self.paths.extend_from_slice(&o.paths);
    }

    pub fn polygons(&self) -> &[Polygon] {
        &self.polygons
    }

    pub fn lines(&self) -> &[Path] {
        &self.paths
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
