pub mod map;

use crate::{
    Diffable,
};

pub enum Edit<'a, T: Diffable<'a>> {
    Copy(&'a T),
    Change(T::D),
}

impl<'a, T: Diffable<'a>> Edit<'a, T> {
    pub fn is_copy(&self) -> bool {
        if let Self::Copy(_) = self {
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
            Self::Copy(value) => map::Edit::Copy(value),
            Self::Change(diff) => map::Edit::Change(diff),
        }
    }
}

pub enum EditSection<T: Eq> {
    Copy(T),
    Add(T),
    Remove(T),
}

impl<T: Eq + std::fmt::Debug> std::fmt::Debug for EditSection<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Copy(value) => write!(f, "Copy({:?})", value),
            Self::Add(value) => write!(f, "Add({:?})", value),
            Self::Remove(value) => write!(f, "Remove({:?})", value),
        }
    }
}

impl<T: Eq + PartialEq> PartialEq for EditSection<T> {
    fn eq(&self, other: &Self) -> bool {
        let left = match self { Self::Copy(left) | Self::Add(left) | Self::Remove(left) => left };
        let right = match other { Self::Copy(right) | Self::Add(right) | Self::Remove(right) => right };
        left == right
    }
}

impl<T: Eq> Eq for EditSection<T> {}
