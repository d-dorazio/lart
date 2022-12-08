use std::ops::{Mul, MulAssign};

use crate::{v, Geometry, Path, Polygon, Rect, V};

#[derive(Debug, Clone)]
pub struct Xform {
    pub a: V,
    pub b: V,
    pub c: V,
}

impl Xform {
    pub fn identity() -> Self {
        Self {
            a: v(1, 0),
            b: v(0, 1),
            c: v(0, 0),
        }
    }

    pub fn xlate(d: V) -> Self {
        Self {
            a: v(1, 0),
            b: v(0, 1),
            c: d,
        }
    }

    pub fn scale(s: V) -> Self {
        Self {
            a: v(s.x, 0.0),
            b: v(0.0, s.y),
            c: v(0.0, 0.0),
        }
    }

    pub fn rot(a: f64) -> Self {
        let (sa, ca) = a.sin_cos();
        Self {
            a: v(ca, sa),
            b: v(-sa, ca),
            c: v(0.0, 0.0),
        }
    }

    pub fn rot_on(p: V, a: f64) -> Self {
        Self::xlate(-p) * Self::rot(a) * Self::xlate(p)
    }

    pub fn rect_to_rect(src: &Rect, dst: &Rect) -> Self {
        let sf = f64::min(dst.width() / src.width(), dst.height() / src.height());

        Self::xlate(-src.center()) * Self::scale(v(sf, sf)) * Self::xlate(dst.center())
    }
}

impl<'a> MulAssign<&'a Xform> for Xform {
    fn mul_assign(&mut self, m: &'a Xform) {
        self.a = self.a.x * m.a + self.a.y * m.b;
        self.b = self.b.x * m.a + self.b.y * m.b;
        self.c = self.c * m;
    }
}

impl<'a> MulAssign<&'a Xform> for V {
    fn mul_assign(&mut self, rhs: &'a Xform) {
        *self = self.x * rhs.a + self.y * rhs.b + rhs.c;
    }
}

impl<'a> MulAssign<&'a Xform> for Path {
    fn mul_assign(&mut self, rhs: &'a Xform) {
        for p in self.iter_mut() {
            *p *= rhs;
        }
    }
}

impl<'a> MulAssign<&'a Xform> for Polygon {
    fn mul_assign(&mut self, rhs: &'a Xform) {
        for a in &mut self.areas {
            *a *= rhs;
        }
    }
}

impl<'a> MulAssign<&'a Xform> for Geometry {
    fn mul_assign(&mut self, rhs: &'a Xform) {
        for p in &mut self.polygons {
            *p *= rhs;
        }
        for p in &mut self.paths {
            *p *= rhs;
        }
    }
}

macro_rules! impl_trivial_xform_helpers {
    ($t: ident) => {
        impl MulAssign<Xform> for $t {
            fn mul_assign(&mut self, rhs: Xform) {
                *self *= &rhs;
            }
        }

        impl Mul<Xform> for $t {
            type Output = Self;

            fn mul(self, rhs: Xform) -> Self::Output {
                self * &rhs
            }
        }

        impl<'a> Mul<&'a Xform> for $t {
            type Output = Self;

            fn mul(mut self, rhs: &'a Xform) -> Self::Output {
                self *= rhs;
                self
            }
        }
    };
}

impl_trivial_xform_helpers!(Xform);
impl_trivial_xform_helpers!(V);
impl_trivial_xform_helpers!(Path);
impl_trivial_xform_helpers!(Polygon);
impl_trivial_xform_helpers!(Geometry);
