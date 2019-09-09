use crate::{edit::collection, edit::Edit, lcs::Lcs, Diffable};

macro_rules! collection_impl {
    ($($typ:ident),*) => {
        $(
            impl<'a, T: Eq + 'a> Diffable<'a> for $typ<T> {
                type D = Box<dyn Iterator<Item = collection::Edit<&'a T>> + 'a>;

                fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
                    let (s, modified) = Lcs::new(
                        self.iter(),
                        || other.iter(),
                        self.len(),
                        other.len(),
                    )
                    .diff(self.iter(), other.iter());

                    if modified {
                        Edit::Change(Box::new(s) as Box<dyn Iterator<Item = _>>)
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
            impl<'a, T: Eq + $key_constraint + 'a> Diffable<'a> for $typ<T> {
                type D = Box<dyn Iterator<Item = collection::Edit<&'a T>> + 'a>;

                fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
                    let (s, modified) = Lcs::new(
                        self.iter(),
                        || other.iter(),
                        self.len(),
                        other.len(),
                    )
                        .diff_unordered(self.iter(), other.iter());

                    if modified {
                        Edit::Change(Box::new(s) as Box<dyn Iterator<Item = _>>)
                    } else {
                        Edit::Copy
                    }
                }
            }
        )*
    }
}

use std::{collections::{BTreeSet, HashSet}, hash::Hash};
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
