use crate::{
    Edit,
    Diffable,
};

impl<'a> Diffable<'a> for i32 {
    type D = (&'a i32, &'a i32);

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
        if self == other {
            Edit::Copy
        } else {
            Edit::Change((self, other))
        }
    }
}

impl<'a> Diffable<'a> for String {
    type D = (&'a str, &'a str);

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
        if self == other {
            Edit::Copy
        } else {
            Edit::Change((self.as_ref(), other.as_ref()))
        }
    }
}

use std::{
    hash::Hash,
    collections::HashMap,
};

type EditedHashMap<'a, K, V> = HashMap<&'a K, Edit<'a, V>>;

impl<
    'a,
    K: Eq + Hash + 'a,
    V: Diffable<'a> + 'a
> Diffable<'a> for HashMap<K, V> {
    type D = EditedHashMap<'a, K, V>;

    fn diff(&'a self, other: &'a Self) -> Edit<Self> {
        let intersection = self.iter()
            .filter_map(|(k, v)| Some((k, (v, other.get(k)?))));

        let unique_self = self.iter()
            .filter(|(k, _)| !other.contains_key(k));

        let unique_other = other.iter()
            .filter(|(k, _)| !self.contains_key(k));

        let value_diffs = unique_other
            .map(|(k, v)| (k, Edit::Insert(v)))
            .chain(
                unique_self
                    .map(|(k, _)| (k, Edit::Remove))
            )
            .chain(
                intersection
                    .map(|(k, (self_v, other_v))| (k, self_v.diff(other_v)))
            )
            .collect::<HashMap<_,_>>();

        if value_diffs.values().any(|v| !v.is_copy()) {
            Edit::Change(value_diffs)
        } else {
            Edit::Copy
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashmap_simple() {
        let unity: HashMap<_,_> = [(1,1), (2,2), (3,3)].iter()
            .cloned().collect();
        let not_unity: HashMap<_,_> = [(1,1), (2,3), (4,4)].iter()
            .cloned().collect();

        if let Edit::Change(diff) = unity.diff(&not_unity) {
            assert!(
                diff.get(&1).unwrap().is_copy()
            );
            assert_eq!(
                diff.get(&2).unwrap().change().unwrap(),
                &(&2, &3)
            );
            assert!(
                diff.get(&3).unwrap().is_remove()
            );
            assert_eq!(
                diff.get(&4).unwrap().insert().unwrap(),
                &4
            );
        } else {
            unreachable!()
        }
    }
}
