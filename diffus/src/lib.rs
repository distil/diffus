pub mod diffable_impls;
pub mod edit;
mod lcs;
pub mod same;

#[cfg(feature = "serialize-impl")]
use serde::Serialize;

macro_rules! diffable {
    (: $($constraints:ident)*) => {
        pub trait Diffable<'a>
        {
            type Diff: 'a $(+$constraints)*;

            fn diff(&'a self, other: &'a Self) -> edit::Edit<Self::Diff>;
        }
    }
}

#[cfg(feature = "serialize-impl")]
diffable!{ : Serialize }
#[cfg(not(feature = "serialize-impl"))]
diffable!{ : }

pub trait Same {
    fn same(&self, other: &Self) -> bool;
}
