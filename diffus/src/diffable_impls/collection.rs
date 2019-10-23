use crate::{
    edit::{collection, Edit},
    Diffable, Same,
};

pub struct CollectionDiff<T>(pub(crate) crate::lcs::LcsResult<T>);

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
    ($($typ:ident),*) => {
        $(
            impl<'a, T: Same + Diffable<'a> + 'a> Diffable<'a> for $typ<T> {
                type Diff = CollectionDiff<collection::Edit<'a, T, T::Diff>>;

                fn diff(&'a self, other: &'a Self) -> Edit<Self::Diff> {

                    if let Some(s) = crate::lcs::enriched_lcs(
                        crate::lcs::c_matrix(
                            self.iter(),
                            || other.iter(),
                            self.len(),
                            other.len(),
                        ),
                        self.iter(),
                        other.iter()) {
                        Edit::Change(CollectionDiff(s))
                    } else {
                        Edit::Copy
                    }
                }
            }
        )*
    }
}

use std::collections::{BinaryHeap, LinkedList, VecDeque};
collection_impl! {
    BinaryHeap, LinkedList, Vec, VecDeque
}

macro_rules! set_impl {
    ($(($typ:ident, $key_constraint:ident)),*) => {
        $(
            impl<'a, T: Same + Diffable<'a> + $key_constraint + 'a> Diffable<'a> for $typ<T> {
                type Diff = CollectionDiff<collection::Edit<'a, T, T::Diff>>;

                fn diff(&'a self, other: &'a Self) -> Edit<Self::Diff> {
                    if let Some(s) = crate::lcs::enriched_lcs_unordered(
                        crate::lcs::c_matrix(
                            self.iter(),
                            || other.iter(),
                            self.len(),
                            other.len(),
                        ),
                        self.iter(),
                        other.iter()) {
                        Edit::Change(CollectionDiff(s))
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
                diff.into_iter().collect::<Vec<_>>(),
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
