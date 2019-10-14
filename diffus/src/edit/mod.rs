pub mod collection;
pub mod enm;
pub mod map;

#[cfg(feature = "serialize-impl")]
use serde::Serialize;


macro_rules! edit {
    (: $($constraints:ident),*) => {
        #[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
        #[derive(Debug, PartialEq)]
        pub enum Edit<Diff: $($constraints)?> {
            Copy,
            Change(Diff),
        }

        impl<Diff: $($constraints)?> Edit<Diff> {
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

        impl<'a,
            Diff: $($constraints)?,
            T: crate::Diffable<'a, Diff = Diff> $(+$constraints)? + 'a
        > Into<map::Edit<'a, T>> for Edit<Diff> {
            fn into(self) -> map::Edit<'a, T> {
                match self {
                    Self::Copy => map::Edit::Copy,
                    Self::Change(diff) => map::Edit::Change(diff),
                }
            }
        }
    }
}

#[cfg(feature = "serialize-impl")]
edit!{ : Serialize }
#[cfg(not(feature = "serialize-impl"))]
edit!{ : }
