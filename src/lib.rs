mod diffable_impls;
mod edit;

pub use edit::Edit;

pub trait Diffable<'a>: Sized {
    type D: Sized + 'a;

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self>;
}
