use crate::{Bbox, Rect, Transform, V};

impl V {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn len(self) -> f64 {
        f64::hypot(self.x, self.y)
    }

    pub fn len2(self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn normalized(self) -> Self {
        self / self.len()
    }

    pub fn dot(self, rhs: V) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn angle(self) -> f64 {
        f64::atan2(self.y, self.x)
    }

    pub fn dist(self, rhs: Self) -> f64 {
        (rhs - self).len()
    }

    pub fn dist2(self, rhs: Self) -> f64 {
        (rhs - self).len2()
    }
}

impl Bbox for V {
    fn bbox(&self) -> Option<Rect> {
        Some(Rect::new(*self))
    }
}

impl Transform for V {
    fn transform(&mut self, f: &mut impl FnMut(V) -> V) {
        *self = f(*self);
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

        impl<W: Into<f64>> std::ops::$tr<(W, W)> for V {
            type Output = Self;
            fn $name(self, rhs: (W, W)) -> Self::Output {
                V::new(self.x.$name(rhs.0.into()), self.y.$name(rhs.1.into()))
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

        impl<W: Into<f64>> std::ops::$tr<(W, W)> for V {
            fn $name(&mut self, rhs: (W, W)) {
                self.x.$name(rhs.0.into());
                self.y.$name(rhs.1.into());
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

impl<T: Into<f64>> From<(T, T)> for V {
    fn from((x, y): (T, T)) -> Self {
        V {
            x: x.into(),
            y: y.into(),
        }
    }
}
