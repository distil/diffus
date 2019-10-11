pub mod collection;
pub mod enm;
pub mod map;

#[cfg(feature = "serde")]
use serde::Serialize;

macro_rules! edit_struct_contstraint {
    ($($constraints:tt),*) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        #[derive(Debug, PartialEq)]
        pub enum Edit<Diff: $($constraints)?> {
            Copy,
            Change(Diff),
        }
    }
}

#[cfg(feature = "serde")]
edit_struct_contstraint!{ serde::Serialize }
#[cfg(not(feature = "serde"))]
edit_struct_contstraint!{ }

impl<Diff> Edit<Diff> {
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

    pub fn change(&self) -> Option<&Diff> {
        if let Self::Change(value_diff) = self {
            Some(value_diff)
        } else {
            None
        }
    }
}

impl<'a, Diff, T: crate::Diffable<'a, Diff = Diff> + 'a> Into<map::Edit<'a, T>> for Edit<Diff> {
    fn into(self) -> map::Edit<'a, T> {
        match self {
            Self::Copy => map::Edit::Copy,
            Self::Change(diff) => map::Edit::Change(diff),
        }
    }
}
