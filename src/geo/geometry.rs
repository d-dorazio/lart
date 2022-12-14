use crate::{bbox_union, Bbox, Geometry, Path, Polygon, Rect};

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

    pub fn is_empty(&self) -> bool {
        self.polygons.iter().all(Polygon::is_empty) && self.paths.iter().all(Path::is_empty)
    }

    pub fn polygons(&self) -> &[Polygon] {
        &self.polygons
    }

    pub fn lines(&self) -> &[Path] {
        &self.paths
    }

    pub fn push_polygon(&mut self, p: Polygon) {
        self.polygons.push(p);
    }

    pub fn push_polygons(&mut self, p: impl IntoIterator<Item = Polygon>) {
        self.polygons.extend(p);
    }

    pub fn push_path(&mut self, p: Path) {
        self.paths.push(p);
    }

    pub fn push_paths(&mut self, p: impl IntoIterator<Item = Path>) {
        self.paths.extend(p);
    }

    pub fn extend(&mut self, o: &Self) {
        self.polygons.extend_from_slice(&o.polygons);
        self.paths.extend_from_slice(&o.paths);
    }

    pub fn bbox(&self) -> Option<Rect> {
        let mut bboxes = self
            .polygons
            .iter()
            .filter_map(Polygon::bbox)
            .chain(self.paths.iter().filter_map(Path::bbox));

        let mut bbox = bboxes.next()?;
        for b in bboxes {
            bbox.union(&b);
        }

        Some(bbox)
    }
}

impl Bbox for Geometry {
    fn bbox(&self) -> Option<Rect> {
        match (bbox_union(&self.polygons), bbox_union(&self.paths)) {
            (None, None) => None,
            (Some(b), None) | (None, Some(b)) => Some(b),
            (Some(mut b1), Some(b2)) => {
                b1.union(&b2);
                Some(b1)
            }
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

impl From<Rect> for Geometry {
    fn from(r: Rect) -> Self {
        Geometry::from(Polygon::from(r))
    }
}
