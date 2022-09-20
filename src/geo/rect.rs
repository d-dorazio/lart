use crate::V;

pub trait Bbox {
    fn bbox(&self) -> Option<Rect>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    min: V,
    max: V,
}

impl Rect {
    pub const fn new(v: V) -> Self {
        Self { min: v, max: v }
    }

    pub fn pad(&mut self, p: f64) {
        let p = p.abs();
        self.min = self.min - p;
        self.max = self.max + p;
    }

    pub fn expand(&mut self, v: V) {
        self.min.x = f64::min(self.min.x, v.x);
        self.min.y = f64::min(self.min.y, v.y);
        self.max.x = f64::max(self.max.x, v.x);
        self.max.y = f64::max(self.max.y, v.y);
    }

    pub fn union(&mut self, bbox: &Rect) {
        self.min.x = f64::min(self.min.x, bbox.min.x);
        self.min.y = f64::min(self.min.y, bbox.min.y);
        self.max.x = f64::max(self.max.x, bbox.max.x);
        self.max.y = f64::max(self.max.y, bbox.max.y);
    }

    pub fn center(&self) -> V {
        (self.min + self.max) / 2.0
    }

    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    pub fn top(&self) -> f64 {
        self.min.y
    }
    pub fn left(&self) -> f64 {
        self.min.x
    }
    pub fn bottom(&self) -> f64 {
        self.max.y
    }
    pub fn right(&self) -> f64 {
        self.max.x
    }

    pub fn min(&self) -> V {
        self.min
    }
    pub fn max(&self) -> V {
        self.max
    }
}

pub fn bbox_union(v: &[impl Bbox]) -> Option<Rect> {
    let mut vs = v.iter().flat_map(Bbox::bbox);
    let mut bbox = vs.next()?;
    for b in vs {
        bbox.union(&b);
    }
    Some(bbox)
}

impl Bbox for Rect {
    fn bbox(&self) -> Option<Rect> {
        Some(self.clone())
    }
}
