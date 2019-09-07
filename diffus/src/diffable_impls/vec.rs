use crate::{
    Diffable,
    Edit,
};


// TODO Is patience_diff possible without the constraint `Hash`?
// ref: https://docs.rs/patience-diff/0.1.0/patience_diff/fn.patience_diff.html


type EditedVec<'a, T> = Vec<Edit<'a, T>>;

impl<
    'a,
    T: Eq + std::hash::Hash + Diffable<'a> + 'a,
> Diffable<'a> for Vec<T> {
    type D = EditedVec<'a, T>;

    fn diff(&'a self, other: &'a Self) -> Edit<Self> {
        use lcs;

        let value_diffs = lcs::LcsTable::new(self, other).diff()
            .iter()
            .map(|value_diff| {
                match value_diff {
                    lcs::DiffComponent::Unchanged(_, _) => Edit::Copy,
                    lcs::DiffComponent::Insertion(a) => Edit::Insert(a),
                    lcs::DiffComponent::Deletion(_) => Edit::Remove,
                }
            })
            .collect::<Vec<_>>();

        if value_diffs.iter().any(|value_diff| !value_diff.is_copy()) {
            Edit::Change(value_diffs)
        } else {
            Edit::Copy
        }
    }
}
