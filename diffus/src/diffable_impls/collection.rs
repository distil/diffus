use crate::{
    edit::{collection, Edit},
    Diffable, Same,
};

macro_rules! collection_impl {
    ($($typ:ident),*) => {
        $(
            impl<'a, T: Same + Diffable<'a> + 'a> Diffable<'a> for $typ<T> {
                type Diff = Vec<collection::Edit<'a, T, T::Diff>>;

                fn diff(&'a self, other: &'a Self) -> Edit<Self::Diff> {

                    let s = crate::lcs::enriched_lcs(
                        crate::lcs::c_matrix(
                            self.iter(),
                            || other.iter(),
                            self.len(),
                            other.len(),
                        ),
                        self.iter(),
                        other.iter())
                        .collect::<Vec<_>>();

                    if s.iter().all(collection::Edit::is_copy) {
                        Edit::Copy
                    } else {
                        Edit::Change(s)
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
