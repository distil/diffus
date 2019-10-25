use crate::Same;

#[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
#[derive(Debug, PartialEq, Eq)]
pub enum Edit<'a, T: ?Sized, Diff> {
    Copy(&'a T),
    Insert(&'a T),
    Remove(&'a T),
    Change(Diff),
}

impl<'a, T: Same + ?Sized, Diff> Edit<'a, T, Diff> {
    pub fn is_copy(&self) -> bool {
        if let Self::Copy(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_insert(&self) -> bool {
        if let Self::Insert(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_remove(&self) -> bool {
        if let Self::Remove(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_change(&self) -> bool {
        self.change().is_some()
    }

    pub fn copy(&self) -> Option<&T> {
        if let Self::Copy(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn insert(&self) -> Option<&T> {
        if let Self::Insert(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn remove(&self) -> Option<&T> {
        if let Self::Remove(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn change(&self) -> Option<&Diff> {
        if let Self::Change(value) = self {
            Some(value)
        } else {
            None
        }
    }
}
