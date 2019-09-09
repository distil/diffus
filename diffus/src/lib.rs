pub mod diffable_impls;
pub mod edit;
mod lcs;

pub use edit::Edit;


pub trait Diffable<'a>: Sized {
    type D: Sized + 'a;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<'a, Self>;
}
