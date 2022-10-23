use crate::{bbox_union, Bbox, Path, Polygon, Rect};

impl Polygon {
    pub const fn new() -> Self {
        Self { areas: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.areas.iter().all(Path::is_empty)
    }
}

impl Bbox for Polygon {
    fn bbox(&self) -> Option<Rect> {
        bbox_union(&self.areas)
    }
}

impl<P: Into<Path>> From<P> for Polygon {
    fn from(p: P) -> Self {
        Self {
            areas: vec![p.into()],
        }
    }
}

impl<P: Into<Path>> FromIterator<P> for Polygon {
    fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
        Polygon {
            areas: iter.into_iter().map(Into::into).collect(),
        }
    }
}
