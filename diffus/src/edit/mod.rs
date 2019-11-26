pub mod collection;
pub mod enm;
pub mod map;
pub mod set;

use crate::Diffable;

#[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
#[derive(Debug, PartialEq, Eq)]
pub enum Edit<'a, T: Diffable<'a> + ?Sized> {
    Copy(&'a T),
    Change(T::Diff),
}

impl<'a, T: Diffable<'a> + ?Sized> Edit<'a, T> {
    pub fn is_copy(&self) -> bool {
        if let Self::Copy(_) = self {
            true
        } else {
            false
        }
    }

    pub fn copy(&self) -> Option<&'a T> {
        if let Self::Copy(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn is_change(&self) -> bool {
        if let Self::Change(_) = self {
            true
        } else {
            false
        }
    }

    pub fn change(&self) -> Option<&T::Diff> {
        if let Self::Change(value_diff) = self {
            Some(value_diff)
        } else {
            None
        }
    }
}

impl<'a, Diff, T: Diffable<'a, Diff = Diff> + 'a> Into<map::Edit<'a, T>> for Edit<'a, T> {
    fn into(self) -> map::Edit<'a, T> {
        match self {
            Self::Copy(value) => map::Edit::Copy(value),
            Self::Change(diff) => map::Edit::Change(diff),
        }
    }
}
