pub mod collection;
pub mod map;
pub mod option;
pub mod primitives;
pub mod set;
pub mod string;

use crate::{edit, Diffable};

impl<'a, T: Diffable<'a> + ?Sized + 'a> Diffable<'a> for &'a T {
    type Diff = T::Diff;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<'a, Self> {
        match (*self).diff(*other) {
            edit::Edit::Change(diff) => edit::Edit::Change(diff),
            edit::Edit::Copy(_) => edit::Edit::Copy(self),
        }
    }
}
