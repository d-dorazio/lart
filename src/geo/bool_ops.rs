use cxx::UniquePtr;

use crate::{
    ffi::{self, new_clipper, Clipper},
    Geometry, Polygon,
};

impl Geometry {
    pub fn buffer(&self, delta: f64) -> Self {
        if delta != 0.0 {
            ffi::buffer(self, delta)
        } else {
            self.clone()
        }
    }
}

macro_rules! bool_op {
    ($tr: ident, $name: ident, $op: ident) => {
        // NOTE: the operation is implemented for non-ref arguments too because
        // it's a bit easier to write when the value is not re-used (i.e. `a -
        // b` is easier to write than `a - &b`).
        impl std::ops::$tr<Geometry> for Geometry {
            type Output = Geometry;

            fn $name(self, rhs: Geometry) -> Self::Output {
                std::ops::$tr::$name(&self, &rhs)
            }
        }

        impl<'a> std::ops::$tr<&'a Geometry> for Geometry {
            type Output = Geometry;

            fn $name(self, rhs: &'a Geometry) -> Self::Output {
                std::ops::$tr::$name(&self, rhs)
            }
        }

        impl std::ops::$tr<Polygon> for Geometry {
            type Output = Geometry;

            fn $name(self, rhs: Polygon) -> Self::Output {
                std::ops::$tr::$name(&self, &rhs)
            }
        }

        impl<'a> std::ops::$tr<Geometry> for &'a Geometry {
            type Output = Geometry;

            fn $name(self, rhs: Geometry) -> Self::Output {
                std::ops::$tr::$name(self, &rhs)
            }
        }

        impl<'a> std::ops::$tr<&'a Geometry> for &'a Geometry {
            type Output = Geometry;

            fn $name(self, rhs: &'a Geometry) -> Self::Output {
                let mut clipper = prepare_op(&self, rhs);
                clipper.pin_mut().$op()
            }
        }

        impl<'a> std::ops::$tr<&'a Polygon> for &'a Geometry {
            type Output = Geometry;

            fn $name(self, rhs: &'a Polygon) -> Self::Output {
                let mut clipper = prepare_poly_op(&self, rhs);
                clipper.pin_mut().$op()
            }
        }
    };
}

bool_op!(BitOr, bitor, union_);
bool_op!(BitAnd, bitand, intersection);
bool_op!(Sub, sub, difference);
bool_op!(BitXor, bitxor, symmetric_difference);

fn prepare_op(lhs: &Geometry, rhs: &Geometry) -> UniquePtr<Clipper> {
    let mut clipper = new_clipper();

    for poly in &lhs.polygons {
        clipper.pin_mut().add_polygon(poly);
    }
    for path in &lhs.paths {
        clipper.pin_mut().add_polyline(path);
    }

    for clip in &rhs.polygons {
        clipper.pin_mut().add_clip(clip);
    }
    // TODO: how to treat open paths in rhs?

    clipper
}

fn prepare_poly_op(lhs: &Geometry, rhs: &Polygon) -> UniquePtr<Clipper> {
    let mut clipper = new_clipper();

    for poly in &lhs.polygons {
        clipper.pin_mut().add_polygon(poly);
    }
    for path in &lhs.paths {
        clipper.pin_mut().add_polyline(path);
    }

    clipper.pin_mut().add_clip(rhs);
    // TODO: how to treat open paths in rhs?

    clipper
}
