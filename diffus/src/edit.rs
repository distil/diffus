use crate::{
    Diffable,
};


pub enum Edit<'a, T: Diffable<'a>> {
    Insert(&'a T),
    Remove,
    Copy, // FIXME needs ref to T when "in the middle" of a diff, see Vec<T> for example
    Change(T::D),
}

impl<'a, T: Diffable<'a>> Edit<'a, T> {
    // FIXME run doctests default
    //
    // Checks if the edit is an insert.
    //
    // # Examples
    //
    // ```
    // assert_eq!(Edit::Insert(&2).is_insert(), true);
    // assert_eq!(Edit::Remove.is_insert(), false);
    // ```
    pub fn is_insert(&self) -> bool {
        if let Self::Insert(_) = self {
            true
        } else {
            false
        }
    }
    // FIXME doc tests
    pub fn is_remove(&self) -> bool {
        if let Self::Remove = self {
            true
        } else {
            false
        }
    }
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
    pub fn insert(&self) -> Option<&'a T> {
        if let Self::Insert(value) = self {
            Some(value)
        } else {
            None
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

