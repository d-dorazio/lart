use crate::{v, V};

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

    pub fn with_dimensions(tl: V, width: f64, height: f64) -> Self {
        let mut b = Self::new(tl);
        b.expand(tl + v(width, height));
        b
    }

    pub fn pad(&mut self, p: f64) {
        let mut r = Rect::new(self.min - p);
        r.expand(self.max + p);
        *self = r;
    }

    pub fn padded(&self, p: f64) -> Self {
        let mut a = self.clone();
        a.pad(p);
        a
    }

    pub fn expand(&mut self, v: V) {
        self.min.x = f64::min(self.min.x, v.x);
        self.min.y = f64::min(self.min.y, v.y);
        self.max.x = f64::max(self.max.x, v.x);
        self.max.y = f64::max(self.max.y, v.y);
    }

    pub fn expanded(&self, v: V) -> Self {
        let mut a = self.clone();
        a.expand(v);
        a
    }

    pub fn union(&mut self, bbox: &Rect) {
        self.min.x = f64::min(self.min.x, bbox.min.x);
        self.min.y = f64::min(self.min.y, bbox.min.y);
        self.max.x = f64::max(self.max.x, bbox.max.x);
        self.max.y = f64::max(self.max.y, bbox.max.y);
    }

    pub fn dist(&self, v: V) -> f64 {
        self.dist2(v).sqrt()
    }

    pub fn dist2(&self, v: V) -> f64 {
        let vx = f64::max(self.min.x - v.x, v.x - self.max.x);
        let vy = f64::max(self.min.y - v.y, v.y - self.max.y);

        f64::max(vx, 0.0).powi(2) * f64::max(vy, 0.0).powi(2)
    }

    pub fn center(&self) -> V {
        (self.min + self.max) / 2.0
    }

    pub fn dimensions(&self) -> V {
        v(self.width(), self.height())
    }

    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    pub fn area(&self) -> f64 {
        self.width() * self.height()
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

    pub fn subdivide(&self, xdivs: u32, ydivs: u32) -> impl Iterator<Item = Rect> + '_ {
        let d = v(self.width(), self.height()) / v(xdivs, ydivs);

        (0..ydivs)
            .flat_map(move |r| (0..xdivs).map(move |c| (r, c)))
            .map(move |(r, c)| Rect::with_dimensions(self.min + d * v(c, r), d.x, d.y))
    }
}

pub fn bbox_union<'a, B: Bbox + 'a>(v: impl IntoIterator<Item = &'a B>) -> Option<Rect> {
    let mut vs = v.into_iter().flat_map(Bbox::bbox);
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
