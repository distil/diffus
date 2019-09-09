pub mod diffable_impls;
mod edit;
mod lcs;

pub trait Diffable<'a>: Sized {
    type D: Sized + 'a;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<'a, Self>;
}
