pub mod collection;
pub mod map;

use crate::Diffable;

pub enum Edit<'a, T: Diffable<'a>> {
    Copy,
    Change(T::D),
}

impl<'a, T: Diffable<'a>> Edit<'a, T> {
    pub fn is_copy(&self) -> bool {
        if let Self::Copy = self {
            true
        } else {
            false
        }
    }

    pub fn is_change(&self) -> bool {
        if let Self::Change(_) = self {
            true
        } else {
            false
        }
    }

    pub fn change(&self) -> Option<&T::D> {
        if let Self::Change(value_diff) = self {
            Some(value_diff)
        } else {
            None
        }
    }
}

impl<'a, T: Diffable<'a>> Into<map::Edit<'a, T>> for Edit<'a, T> {
    fn into(self) -> map::Edit<'a, T> {
        match self {
            Self::Copy => map::Edit::Copy,
            Self::Change(diff) => map::Edit::Change(diff),
        }
    }
}
