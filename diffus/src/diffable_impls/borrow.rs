use crate::{edit, Diffable};
use std::borrow::Borrow;

fn diff_borrowable<'a, T, C, D>(left: &'a C, right: &'a C) -> edit::Edit<'a, C>
where
    T: Diffable<'a> + ?Sized + 'a,
    C: Borrow<T> + Diffable<'a, Diff = D> + ?Sized,
    D: From<T::Diff>
{
    match left.borrow().diff(right.borrow()) {
        edit::Edit::Copy(_) => edit::Edit::Copy(left),
        edit::Edit::Change(diff) => edit::Edit::Change(diff.into()),
    }
}

macro_rules! borrow_impl {
    ($($typ:ident),*) => {
        $(
            impl<'a, T: Diffable<'a> + ?Sized + 'a> Diffable<'a> for $typ<T> {
                type Diff = $typ<T::Diff>;

                fn diff(&'a self, other: &'a Self) -> edit::Edit<'a, Self> {
                    diff_borrowable::<T, _, _>(self, other)
                }
            }
        )*
    }
}

use std::{rc::Rc, sync::Arc};
borrow_impl! {
    Box, Rc, Arc
}

impl<'a, T: Diffable<'a> + ?Sized + 'a> Diffable<'a> for &'a T {
    type Diff = T::Diff;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<'a, Self> {
        diff_borrowable::<T, _, _>(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_example() {
        let left = 13;
        let right = 37;

        if let edit::Edit::Change(diff) = Box::new(left).diff(&Box::new(right)) {
            assert_eq!(*diff, (&13, &37));
        }
    }

    #[test]
    fn rc_example() {
        let left = 13;
        let right = 37;

        if let edit::Edit::Change(diff) = Rc::new(left).diff(&Rc::new(right)) {
            assert_eq!(*diff, (&13, &37));
        }
    }

    #[test]
    fn arc_example() {
        let left = 13;
        let right = 37;

        if let edit::Edit::Change(diff) = Arc::new(left).diff(&Arc::new(right)) {
            assert_eq!(*diff, (&13, &37));
        }
    }

    #[test]
    fn reference_example() {
        let left = 13;
        let right = 37;

        if let edit::Edit::Change(diff) = (&left).diff(&(&right)) {
            assert_eq!(diff, (&13, &37));
        }
    }
}
