use crate::Same;

// FIXME references ?
// FIXME T::D ?
#[derive(Debug, PartialEq, Eq)]
pub enum Edit<T, D> {
    Copy(T),
    Change(D),
    Insert(T),
    Remove(T),
}

impl<T: Same, D> Edit<T, D> {
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

    pub fn change(&self) -> Option<&D> {
        if let Self::Change(value) = self {
            Some(value)
        } else {
            None
        }
    }
}
