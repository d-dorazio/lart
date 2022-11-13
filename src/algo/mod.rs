pub mod chaikin;
pub mod circle_packing;
mod simplify;
pub mod spline;
pub mod voro_tri;

use std::f64::consts::TAU;

pub use chaikin::*;
pub use circle_packing::*;
pub use simplify::*;
pub use voro_tri::*;

use crate::V;

/// Return the `n` angles diving the circle in `n` arcs.
pub fn polar_angles(n: u16) -> impl Iterator<Item = f64> {
    (0..n).map(move |i| {
        let t = f64::from(i) / f64::from(n);
        t * TAU
    })
}

/// Return an iterator over the positions in a `columns` x `rows` grid.
///
/// Note that the positions are normalized in [0..1]
pub fn grid_positions(columns: u16, rows: u16) -> impl Iterator<Item = V> {
    (0..rows).flat_map(move |r| {
        (0..columns).map(move |c| {
            V::new(
                f64::from(c) / f64::from(columns),
                f64::from(r) / f64::from(rows),
            )
        })
    })
}
