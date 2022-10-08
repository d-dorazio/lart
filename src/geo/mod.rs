pub mod bool_ops;
pub mod geometry;
pub mod path;
pub mod polygon;
pub mod rect;
pub(crate) mod types;
pub mod utils;
pub mod v;

pub use rect::*;
pub use types::*;
pub use utils::*;
pub use v::v;

pub trait Transform: Sized + Clone {
    fn transform(&mut self, f: &mut impl FnMut(V) -> V);

    fn transformed(&self, f: &mut impl FnMut(V) -> V) -> Self {
        let mut out = self.clone();
        out.transform(f);
        out
    }
}
