use cxx::UniquePtr;

use crate::{
    ffi::{self, new_clipper, Clipper},
    Geometry,
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
        impl<G: Into<Geometry>> std::ops::$tr<G> for Geometry {
            type Output = Geometry;

            fn $name(self, rhs: G) -> Self::Output {
                let mut clipper = prepare_op(&self, &rhs.into());
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
