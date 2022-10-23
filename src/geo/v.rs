use std::ops::Neg;

use rand::{distributions::uniform::SampleRange, Rng};

use crate::{Bbox, Rect, V};

pub fn v(x: impl Into<f64>, y: impl Into<f64>) -> V {
    V::new(x.into(), y.into())
}

impl V {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn polar(a: f64, r: f64) -> Self {
        let (s, c) = a.sin_cos();
        Self::new(c * r, s * r)
    }

    pub fn norm(self) -> f64 {
        f64::hypot(self.x, self.y)
    }

    pub fn norm2(self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn normalized(self) -> Self {
        self / self.norm()
    }

    pub fn dot(self, rhs: V) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn angle(self) -> f64 {
        f64::atan2(self.y, self.x)
    }

    pub fn dist(self, rhs: Self) -> f64 {
        (rhs - self).norm()
    }

    pub fn dist2(self, rhs: Self) -> f64 {
        (rhs - self).norm2()
    }

    pub fn in_range(
        rng: &mut impl Rng,
        x: impl SampleRange<f64>,
        y: impl SampleRange<f64>,
    ) -> Self {
        Self::new(rng.gen_range(x), rng.gen_range(y))
    }

    pub fn in_rect(rng: &mut impl Rng, rect: &Rect) -> V {
        Self::in_range(rng, rect.left()..=rect.right(), rect.top()..=rect.bottom())
    }
}

impl Bbox for V {
    fn bbox(&self) -> Option<Rect> {
        Some(Rect::new(*self))
    }
}

macro_rules! impl_num_op {
    ($tr: ident, $name: ident) => {
        impl std::ops::$tr<f64> for V {
            type Output = Self;
            fn $name(self, rhs: f64) -> Self::Output {
                V::new(self.x.$name(rhs), self.y.$name(rhs))
            }
        }

        impl std::ops::$tr<V> for f64 {
            type Output = V;
            fn $name(self, rhs: V) -> Self::Output {
                V::new(self.$name(rhs.x), self.$name(rhs.y))
            }
        }

        impl std::ops::$tr<V> for V {
            type Output = Self;
            fn $name(self, rhs: V) -> Self::Output {
                V::new(self.x.$name(rhs.x), self.y.$name(rhs.y))
            }
        }
    };

    (Assign, $tr: ident, $name: ident) => {
        impl std::ops::$tr<f64> for V {
            fn $name(&mut self, rhs: f64) {
                self.x.$name(rhs);
                self.y.$name(rhs);
            }
        }

        impl std::ops::$tr<V> for V {
            fn $name(&mut self, rhs: V) {
                self.x.$name(rhs.x);
                self.y.$name(rhs.y);
            }
        }
    };
}

impl_num_op!(Add, add);
impl_num_op!(Sub, sub);
impl_num_op!(Mul, mul);
impl_num_op!(Div, div);
impl_num_op!(Rem, rem);

impl_num_op!(Assign, AddAssign, add_assign);
impl_num_op!(Assign, SubAssign, sub_assign);
impl_num_op!(Assign, MulAssign, mul_assign);
impl_num_op!(Assign, DivAssign, div_assign);
impl_num_op!(Assign, RemAssign, rem_assign);

impl Neg for V {
    type Output = V;

    fn neg(self) -> Self::Output {
        V::new(-self.x, -self.y)
    }
}
