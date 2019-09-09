pub mod diffable_impls;
mod edit;
mod lcs;

pub use edit::{Edit, EditField, EditSection};

pub trait Diffable<'a>: Sized {
    type D: Sized + 'a;

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self>;
}
