use voronator::{delaunator, VoronoiDiagram};

use crate::{Geometry, Path, Polygon, V};

pub fn triangulate(pts: impl AsRef<[V]>) -> Geometry {
    let pts = pts.as_ref();

    let tri = delaunator::triangulate(pts).map_or_else(Vec::new, |t| t.triangles);

    let polys = tri
        .chunks_exact(3)
        .map(|vs| {
            let a = pts[vs[0]];
            let b = pts[vs[1]];
            let c = pts[vs[2]];

            Polygon::from(vec![a, b, c])
        })
        .collect();

    Geometry::from_polygons(polys)
}

// NOTE: clip must be in clockwise order in order for the clipping to work
pub fn voronoi(pts: impl AsRef<[V]>, clip: &Path) -> Geometry {
    let pts = pts.as_ref();

    let polys = VoronoiDiagram::with_bounding_polygon(
        pts.to_vec(),
        &voronator::polygon::Polygon::from_points(clip.points.clone()),
    )
    .map_or_else(Vec::new, |res| {
        res.cells()
            .iter()
            .map(|p| Polygon::from(p.points().to_vec()))
            .collect()
    });

    Geometry::from_polygons(polys)
}

impl delaunator::Coord for V {
    fn from_xy(x: f64, y: f64) -> Self {
        V::new(x, y)
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl delaunator::Vector<V> for V {}
