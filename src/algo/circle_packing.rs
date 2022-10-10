use rand::Rng;

use crate::{Rect, V};

pub struct CirclePacker {
    circles: Vec<(V, f64)>,
    pub bbox: Rect,
    pub margin: f64,
    pub min_radius: f64,
    pub max_radius: f64,
}

impl CirclePacker {
    pub fn new(bbox: Rect) -> Self {
        let max_radius = f64::min(bbox.width(), bbox.height()) / 2.0;
        Self {
            circles: vec![],
            bbox,
            margin: 0.0,
            min_radius: 0.0,
            max_radius,
        }
    }

    pub fn circles(&self) -> &[(V, f64)] {
        &self.circles
    }

    pub fn generate(&mut self, rng: &mut impl Rng) {
        let mut bbox = self.bbox.clone();
        bbox.pad(-self.margin);

        let c = V::in_rect(rng, &bbox);
        let mut r = f64::min(
            f64::min(self.bbox.right() - c.x, c.x - self.bbox.left()),
            f64::min(self.bbox.bottom() - c.y, c.y - self.bbox.top()),
        );
        if r < self.min_radius {
            return;
        }

        for &(cc, rr) in &self.circles {
            let d = cc.dist(c) - rr - self.margin;
            if d < r {
                r = d;
                if r < self.min_radius {
                    return;
                }
            }
        }

        r = r.min(self.max_radius - self.margin);
        self.circles.push((c, r));
    }
}
