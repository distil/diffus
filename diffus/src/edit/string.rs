use crate::lcs;

#[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Edit {
    Copy(char),
    Insert(char),
    Remove(char),
}

impl From<lcs::Edit<char>> for Edit {
    fn from(edit: lcs::Edit<char>) -> Self {
        use lcs::Edit::*;
        match edit {
            Same(left, _) => Self::Copy(left),
            Insert(value) => Self::Insert(value),
            Remove(value) => Self::Remove(value),
        }
    }
}

impl Edit {
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

    pub fn copy(self) -> Option<char> {
        if let Self::Copy(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn insert(self) -> Option<char> {
        if let Self::Insert(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn remove(self) -> Option<char> {
        if let Self::Remove(value) = self {
            Some(value)
        } else {
            None
        }
    }
}
