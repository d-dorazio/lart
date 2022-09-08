use crate::{Path, Polygon};

impl<P: Into<Path>> From<P> for Polygon {
    fn from(p: P) -> Self {
        Self {
            areas: vec![p.into()],
        }
    }
}
