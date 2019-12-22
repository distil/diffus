use std::borrow::Borrow;
use crate::{edit, Diffable};

macro_rules! borrow_impl {
    ($($typ:ident),*) => {
        $(
            impl<'a, T: Diffable<'a> + ?Sized + 'a> Diffable<'a> for $typ<T> {
                type Diff = T::Diff;

                fn diff(&'a self, other: &'a Self) -> edit::Edit<'a, Self> {
                    let self_: &T = self.borrow();
                    let other: &T = other.borrow();
                    match self_.diff(other) {
                        edit::Edit::Copy(_) => edit::Edit::Copy(self),
                        edit::Edit::Change(diff) => edit::Edit::Change(diff),
                    }
                }
            }
        )*
    }
}

use std::{
    rc::Rc,
    sync::Arc,
};
borrow_impl! {
    Box, Rc, Arc
}

impl<'a, T: Diffable<'a> + ?Sized + 'a> Diffable<'a> for &'a T {
    type Diff = T::Diff;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<'a, Self> {
        match (*self).diff(*other) {
            edit::Edit::Copy(_) => edit::Edit::Copy(self),
            edit::Edit::Change(diff) => edit::Edit::Change(diff),
        }
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
            assert_eq!(
                (left, right),
                (&13, &37)
            );
        }
    }

    #[test]
    fn rc_example() {
        let left = 13;
        let right = 37;

        if let edit::Edit::Change((left, right)) = Rc::new(left).diff(&Rc::new(right)) {
            assert_eq!(
                (left, right),
                (&13, &37)
            );
        }
    }

    #[test]
    fn arc_example() {
        let left = 13;
        let right = 37;

        if let edit::Edit::Change((left, right)) = Arc::new(left).diff(&Arc::new(right)) {
            assert_eq!(
                (left, right),
                (&13, &37)
            );
        }
    }

    #[test]
    fn reference_example() {
        let left = 13;
        let right = 37;

        if let edit::Edit::Change((left, right)) = (&left).diff(&(&right)) {
            assert_eq!(
                (left, right),
                (&13, &37)
            );
        }
    }



}
