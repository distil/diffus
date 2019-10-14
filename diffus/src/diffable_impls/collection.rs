use crate::{
    edit::{collection, Edit},
    lcs::Lcs,
    Diffable, Same,
};

#[cfg(feature = "serialize-impl")]
use serde::Serialize;


#[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
pub struct CollectionDiff<T>(crate::lcs::LcsResult<T>);

pub struct IntoIter<T>(<crate::lcs::LcsResult<T> as std::iter::IntoIterator>::IntoIter);

impl<T> std::iter::IntoIterator for CollectionDiff<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }

}

impl<T> std::iter::Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

macro_rules! collection_impl {
    ($typ:ident : $($constraint:ident),*) => {
        impl<'a, T: Same $(+$constraint)* + Diffable<'a> + 'a> Diffable<'a> for $typ<T> {
            // FIXME check if possible to do more generic
            type Diff = CollectionDiff<collection::Edit<&'a T, T::Diff>>;

            fn diff(&'a self, other: &'a Self) -> Edit<Self::Diff> {
                let (s, modified) = Lcs::new(
                    self.iter(),
                    || other.iter(),
                    self.len(),
                    other.len(),
                )
                    .diff(self.iter(), other.iter());

                if modified {
                    Edit::Change(CollectionDiff(s))
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
    (($typ:ident, $key_constraint:ident) : $($constraint:ident),* ) => {
        impl<'a, T: Same + Diffable<'a> + $key_constraint $(+$constraint)* + 'a> Diffable<'a> for $typ<T> {
            type Diff = CollectionDiff<collection::Edit<&'a T, T::Diff>>;

            fn diff(&'a self, other: &'a Self) -> Edit<Self::Diff> {
                let (s, modified) = Lcs::new(
                    self.iter(),
                    || other.iter(),
                    self.len(),
                    other.len(),
                )
                    .diff_unordered(self.iter(), other.iter());

                if modified {
                    Edit::Change(CollectionDiff(s))
                } else {
                    Edit::Copy
                }
            }
        }
    }
}

use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
};

#[cfg(feature = "serialize-impl")]
set_impl! { (BTreeSet, Hash) : Serialize }
#[cfg(feature = "serialize-impl")]
set_impl! { (HashSet, Hash) : Serialize }
#[cfg(not(feature = "serialize-impl"))]
set_impl! { (BTreeSet, Hash) : }
#[cfg(not(feature = "serialize-impl"))]
set_impl! { (HashSet, Hash) : }


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
