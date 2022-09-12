#[cxx::bridge]
pub mod ffi {
    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    pub struct V {
        x: f64,
        y: f64,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Path {
        points: Vec<V>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Polygon {
        areas: Vec<Path>,
    }

    #[derive(Debug, Clone)]
    pub struct Geometry {
        polygons: Vec<Polygon>,
        paths: Vec<Path>,
    }

    unsafe extern "C++" {
        include!("lart/include/lart.h");

        type Clipper;

        fn new_clipper() -> UniquePtr<Clipper>;
        fn add_polygon(self: Pin<&mut Clipper>, polygon: &Polygon);
        fn add_polyline(self: Pin<&mut Clipper>, polyline: &Path);
        fn add_clip(self: Pin<&mut Clipper>, polygon: &Polygon);

        fn union_(self: Pin<&mut Clipper>) -> Geometry;
        fn intersection(self: Pin<&mut Clipper>) -> Geometry;
        fn difference(self: Pin<&mut Clipper>) -> Geometry;
        fn symmetric_difference(self: Pin<&mut Clipper>) -> Geometry;

        fn buffer(geo: &Geometry, delta: f64) -> Geometry;
    }
}

pub use ffi::*;
