pub mod diffable_impls;
pub mod edit;
pub mod same;
mod lcs;

pub trait Diffable<'a>: Sized {
    type D: Sized + 'a;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<'a, Self>;
}

pub trait Same {
    fn same(&self, other: &Self) -> bool;
}
