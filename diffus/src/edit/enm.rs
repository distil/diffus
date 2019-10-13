#[cfg(feature = "serialize-impl")]
use serde::Serialize;

macro_rules! edit {
    (: $($constraints:ident),*) => {
        // TODO figure out the reason why it's not Edit<T> { .., AssociatedChanged(T::D) }
        // FIXME T: Same?
        #[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
        #[derive(Debug, PartialEq)]
        pub enum Edit<'a, T: $($constraints)?, Diff: $($constraints)?> {
            Copy,
            VariantChanged(&'a T, &'a T),
            AssociatedChanged(Diff),
        }

        impl<'a, T: $($constraints)?, Diff: $($constraints)?> Edit<'a, T, Diff> {
            pub fn is_copy(&self) -> bool {
                if let Self::Copy = self {
                    true
                } else {
                    false
                }
            }

            pub fn is_variant_changed(&self) -> bool {
                if let Self::VariantChanged(_, _) = self {
                    true
                } else {
                    false
                }
            }

            pub fn is_associated_changed(&self) -> bool {
                if let Self::AssociatedChanged(_) = self {
                    true
                } else {
                    false
                }
            }

            pub fn variant_changed(&self) -> Option<(&'a T, &'a T)> {
                if let Self::VariantChanged(left, right) = self {
                    Some((left, right))
                } else {
                    None
                }
            }

            pub fn associated_change(&self) -> Option<&Diff> {
                if let Self::AssociatedChanged(value) = self {
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
