use crate::Same;

#[cfg(feature = "serialize-impl")]
use serde::Serialize;


macro_rules! edit {
    (: $($constraints:ident),*) => {
        // FIXME remove Debug
        // FIXME T: Same?
        #[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
        #[derive(Debug, PartialEq, Eq)]
        pub enum Edit<T: $($constraints)?, Diff: $($constraints)?> {
            Copy(T),
            Change(Diff),
            Insert(T),
            Remove(T),
        }

        impl<T: Same $(+$constraints)*, Diff: $($constraints)?> Edit<T, Diff> {
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

            pub fn change(&self) -> Option<&Diff> {
                if let Self::Change(value) = self {
                    Some(value)
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(feature = "serialize-impl")]
edit!{ : Serialize }
#[cfg(not(feature = "serialize-impl"))]
edit!{ : }
