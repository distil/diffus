pub mod diffable_impls;
pub mod edit;
mod lcs;
pub mod same;

pub trait Diffable<'a>
{
    type Diff: 'a;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<Self::Diff>;
}

pub trait Same {
    fn same(&self, other: &Self) -> bool;
}
