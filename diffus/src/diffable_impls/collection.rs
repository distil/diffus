use crate::{
    edit::{collection, Edit},
    lcs::Lcs,
    Diffable, Same,
};

#[cfg(feature = "serialize-impl")]
use serde::Serialize;

// FIXME ignore successive elements in repetition
macro_rules! collection_impl {
    ($typ:ident : $($constraint:ident),*) => {
        impl<'a, T: Same $(+$constraint)* + Diffable<'a> + 'a> Diffable<'a> for $typ<T> {
            // FIXME check if possible to do more generic
            type Diff = std::collections::vec_deque::IntoIter<collection::Edit<&'a T, T::Diff>>;

            fn diff(&'a self, other: &'a Self) -> Edit<Self::Diff> {
                let (s, modified) = Lcs::new(
                    self.iter(),
                    || other.iter(),
                    self.len(),
                    other.len(),
                )
                    .diff(self.iter(), other.iter());

                if modified {
                    Edit::Change(s)
                } else {
                    Edit::Copy
                }
            }
        }
    }
}

use std::collections::{BinaryHeap, LinkedList, VecDeque};

#[cfg(feature = "serialize-impl")]
collection_impl! { BinaryHeap : Serialize }
#[cfg(feature = "serialize-impl")]
collection_impl! { LinkedList : Serialize }
#[cfg(feature = "serialize-impl")]
collection_impl! { Vec : Serialize }
#[cfg(feature = "serialize-impl")]
collection_impl! { VecDeque : Serialize }
#[cfg(not(feature = "serialize-impl"))]
collection_impl! { BinaryHeap : }
#[cfg(not(feature = "serialize-impl"))]
collection_impl! { LinkedList : }
#[cfg(not(feature = "serialize-impl"))]
collection_impl! { Vec : }
#[cfg(not(feature = "serialize-impl"))]
collection_impl! { VecDeque : }

// FIXME continue here
macro_rules! set_impl {
    ($(($typ:ident, $key_constraint:ident)),*) => {
        $(
            impl<'a, T: Same + Diffable<'a> + $key_constraint + 'a> Diffable<'a> for $typ<T> {
                type Diff = std::collections::vec_deque::IntoIter<collection::Edit<&'a T, T::Diff>>;

                fn diff(&'a self, other: &'a Self) -> Edit<Self::Diff> {
                    let (s, modified) = Lcs::new(
                        self.iter(),
                        || other.iter(),
                        self.len(),
                        other.len(),
                    )
                        .diff_unordered(self.iter(), other.iter());

                    if modified {
                        Edit::Change(s)
                    } else {
                        Edit::Copy
                    }
                }
            }
        )*
    }
}

use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
};
set_impl! {
    (BTreeSet, Hash),
    (HashSet, Hash)
}


#[cfg(test)]
mod tests {
    use super::{collection::Edit::*, *};

    #[test]
    fn diff() {
        use super::Diffable;

        let left = b"XMJYAUZ".to_vec();
        let right = b"MZJAWXU".to_vec();

        let diff = left.diff(&right);
        if let Edit::Change(diff) = diff {
            assert_eq!(
                diff.collect::<Vec<_>>(),
                vec![
                    Remove(&b'X'),
                    Copy(&b'M'),
                    Insert(&b'Z'),
                    Copy(&b'J'),
                    Remove(&b'Y'),
                    Copy(&b'A'),
                    Insert(&b'W'),
                    Insert(&b'X'),
                    Copy(&b'U'),
                    Remove(&b'Z')
                ]
            );
        } else {
            unreachable!()
        }
    }
}
