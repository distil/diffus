use crate::{edit, edit::Edit, Diffable};

enum EditedOption<'a, T> {
    Some(T::D),
    None,
}

impl<'a, T: Diffable<'a> + 'a> Diffable<'a> for Option<T> {
    type D = edit::enm::Edit<'a, Self, EditedOption<'a, T>>;

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {

        match (self, other) {
            (None, None) => Edit::Copy,
            (Some(self_value), Some(self_other)) => {
                let value_diff = self_value.diff(self_other);

                if let Edit::Copy = value_diff {
                    Edit::Copy
                } else {
                    edit::enm::Change(Some(value_diff)) // VALUES DIFFER
                }
            }
            (_, _) => Edit::Copy, // KEYS DIFFER
        }
    }
}
