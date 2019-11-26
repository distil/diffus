use crate::{
    edit::{set, Edit},
    Diffable,
};

macro_rules! set_impl {
    ($(($typ:ident, $key_constraint:ident, $diff_type:ident)),*) => {
        $(
            impl<'a, K: Diffable<'a> + Eq + $key_constraint + 'a> Diffable<'a> for $typ<K> {
                type Diff = $diff_type<&'a K, set::Edit<'a, K>>;

                fn diff(&'a self, other: &'a Self) -> Edit<Self> {
                    let intersection = self
                        .iter()
                        .filter(|k| other.contains(*k));

                    let unique_self = self.iter().filter(|k| !other.contains(*k));

                    let unique_other = other.iter().filter(|k| !self.contains(*k));

                    let value_diffs = unique_other
                        .map(|k| (k, set::Edit::Insert(k)))
                        .chain(unique_self.map(|k| (k, set::Edit::Remove(k))))
                        .chain(intersection.map(|k| (k, set::Edit::Copy(k))))
                        .collect::<$diff_type<_, _>>();

                    if value_diffs.iter().any(|(_, edit)| !edit.is_copy()) {
                        Edit::Change(value_diffs)
                    } else {
                        Edit::Copy(self)
                    }
                }
            }
        )*
    }
}

use std::{
    collections::{
        BTreeSet,
        BTreeMap,
        HashSet,
        HashMap,
    },
    hash::Hash,
};
set_impl! {
    (BTreeSet, Ord, BTreeMap),
    (HashSet, Hash, HashMap)
}

#[cfg(feature = "indexmap-impl")]
use indexmap::{IndexSet, IndexMap};
#[cfg(feature = "indexmap-impl")]
set_impl! { (IndexSet, Hash, IndexMap) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let unity: std::collections::HashSet<_, _> =
            [1, 2, 3].iter().cloned().collect();
        let not_unity: std::collections::HashSet<_, _> =
            [1, 2, 4].iter().cloned().collect();

        if let Edit::Change(diff) = unity.diff(&not_unity) {
            assert!(diff[&1].is_copy());
            assert!(diff[&2].is_copy());
            assert!(diff[&3].is_remove());
            assert_eq!(diff[&4].insert().unwrap(), &4);
        } else {
            unreachable!()
        }
    }
}
