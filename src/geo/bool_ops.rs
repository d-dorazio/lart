use crate::{
    ffi::{self, new_clipper},
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

// probably not the best way to do all of this way (at least I hope so), but
// eh...
macro_rules! bool_op_body {
    (Geometry, Geometry, $lhs: ident, $rhs: ident, $op: ident) => {{
        let mut clipper = new_clipper();

        for poly in &$lhs.polygons {
            clipper.pin_mut().add_polygon(poly);
        }
        for path in &$lhs.paths {
            clipper.pin_mut().add_polyline(path);
        }

        for clip in &$rhs.polygons {
            clipper.pin_mut().add_clip(clip);
        }

        clipper.pin_mut().$op()
    }};

    (Geometry, Polygon, $lhs: ident, $rhs: ident, $op: ident) => {{
        let mut clipper = new_clipper();

        for poly in &$lhs.polygons {
            clipper.pin_mut().add_polygon(poly);
        }
        for path in &$lhs.paths {
            clipper.pin_mut().add_polyline(path);
        }

        clipper.pin_mut().add_clip(&$rhs);

        clipper.pin_mut().$op()
    }};

    (Polygon, Geometry, $lhs: ident, $rhs: ident, $op: ident) => {{
        let mut clipper = new_clipper();

        clipper.pin_mut().add_polygon(&$lhs);

        for poly in &$rhs.polygons {
            clipper.pin_mut().add_clip(poly);
        }

        clipper.pin_mut().$op()
    }};

    (Polygon, Polygon, $lhs: ident, $rhs: ident, $op: ident) => {{
        let mut clipper = new_clipper();

        clipper.pin_mut().add_polygon(&$lhs);
        clipper.pin_mut().add_clip(&$rhs);

        clipper.pin_mut().$op()
    }};
}

macro_rules! bool_op {
    ($tr: ident, $fun_name: ident, $op: ident) => {
        bool_op!(Geometry, Geometry, $tr, $fun_name, $op);
        bool_op!(Geometry, Polygon, $tr, $fun_name, $op);
        bool_op!(Polygon, Geometry, $tr, $fun_name, $op);
        bool_op!(Polygon, Polygon, $tr, $fun_name, $op);
    };

    ($t: ident, $arg: ident, $tr: ident, $fun_name: ident, $op: ident) => {
        impl std::ops::$tr<$arg> for $t {
            type Output = Geometry;

            fn $fun_name(self, rhs: $arg) -> Self::Output {
                std::ops::$tr::$fun_name(&self, &rhs)
            }
        }

        impl<'a> std::ops::$tr<&'a $arg> for $t {
            type Output = Geometry;

            fn $fun_name(self, rhs: &'a $arg) -> Self::Output {
                std::ops::$tr::$fun_name(&self, rhs)
            }
        }

        impl<'a> std::ops::$tr<$arg> for &'a $t {
            type Output = Geometry;

            fn $fun_name(self, rhs: $arg) -> Self::Output {
                std::ops::$tr::$fun_name(self, &rhs)
            }
        }

        impl<'a> std::ops::$tr<&'a $arg> for &'a $t {
            type Output = Geometry;

            fn $fun_name(self, rhs: &'a $arg) -> Self::Output {
                bool_op_body!($t, $arg, self, rhs, $op)
            }
        }
    };
}

bool_op!(BitOr, bitor, union_);
bool_op!(BitAnd, bitand, intersection);
bool_op!(Sub, sub, difference);
bool_op!(BitXor, bitxor, symmetric_difference);
