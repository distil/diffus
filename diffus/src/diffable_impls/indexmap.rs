use crate::{
    edit::{map, Edit},
    Diffable,
};

extern crate indexmap; // FIXME remove extern crate
use indexmap::IndexMap;

type EditedIndexMap<'a, K, V> = IndexMap<&'a K, Edit<'a, V>>;

impl<'a, K: Eq + std::hash::Hash + std::borrow::Borrow<&'a K> + 'a, V: Diffable<'a> + 'a>
    Diffable<'a> for IndexMap<K, V>
{
    type D = EditedIndexMap<'a, K, V>;

    fn diff(&'a self, other: &'a Self) -> Edit<Self> {
        // FIXME code duplication
        let intersection = self
            .iter()
            .filter_map(|(k, v)| Some((k, (v, other.get(k)?))));

        let unique_self = self.iter().filter(|(k, _)| !other.contains_key(k));

        let unique_other = other.iter().filter(|(k, _)| !self.contains_key(k));

        let value_diffs = unique_other
            .map(|(k, v)| (k, map::Edit::Insert(v)))
            .chain(unique_self.map(|(k, _)| (k, map::Edit::Remove)))
            .chain(intersection.map(|(k, (self_v, other_v))| (k, self_v.diff(other_v))))
            .collect::<IndexMap<_, _>>();

        if value_diffs.values().any(|v| !v.is_copy()) {
            Edit::Change(value_diffs)
        } else {
            Edit::Copy
        }
    }
}
