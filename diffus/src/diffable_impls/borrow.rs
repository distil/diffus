use crate::{edit, Diffable};
use std::borrow::Borrow;

fn diff_borrowable<'a, T, C>(left: &'a C, right: &'a C) -> edit::Edit<'a, C>
where
    T: Diffable<'a> + ?Sized + 'a,
    C: Borrow<T> + Diffable<'a, Diff = T::Diff> + ?Sized,
{
    match left.borrow().diff(right.borrow()) {
        edit::Edit::Copy(_) => edit::Edit::Copy(left),
        edit::Edit::Change(diff) => edit::Edit::Change(diff),
    }
}

macro_rules! borrow_impl {
    ($($typ:ident),*) => {
        $(
            impl<'a, T: Diffable<'a> + ?Sized + 'a> Diffable<'a> for $typ<T> {
                type Diff = T::Diff;

                fn diff(&'a self, other: &'a Self) -> edit::Edit<'a, Self> {
                    diff_borrowable::<T, Self>(self, other)
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
        diff_borrowable::<T, Self>(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_example() {
        let left = 13;
        let right = 37;

        if let edit::Edit::Change((left, right)) = Box::new(left).diff(&Box::new(right)) {
            assert_eq!((left, right), (&13, &37));
        }
    }

    #[test]
    fn rc_example() {
        let left = 13;
        let right = 37;

        if let edit::Edit::Change((left, right)) = Rc::new(left).diff(&Rc::new(right)) {
            assert_eq!((left, right), (&13, &37));
        }
    }

    #[test]
    fn arc_example() {
        let left = 13;
        let right = 37;

        if let edit::Edit::Change((left, right)) = Arc::new(left).diff(&Arc::new(right)) {
            assert_eq!((left, right), (&13, &37));
        }
    }

    #[test]
    fn reference_example() {
        let left = 13;
        let right = 37;

        if let edit::Edit::Change((left, right)) = (&left).diff(&(&right)) {
            assert_eq!((left, right), (&13, &37));
        }
    }
}
