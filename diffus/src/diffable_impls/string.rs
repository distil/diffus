use crate::{
    Diffable,
    Edit,
};


impl<'a> Diffable<'a> for String {
    type D = Vec<Edit<'a, String>>;

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
        let difference::Changeset {
            diffs: value_diffs,
            split: _,
            distance: edit_distance,
        }  = difference::Changeset::new(self, other, "");

        let value_diffs = value_diffs.iter().map(|value_diff| {
            match value_diff {
                difference::Difference::Same(_) => Edit::Copy,
                difference::Difference::Rem(_) => Edit::Remove,
                difference::Difference::Add(a) => Edit::Insert(a),
            }
        })
        .collect::<Vec<_>>();

        if edit_distance > 0 {
            Edit::Change(value_diffs)
        } else {
            Edit::Copy
        }
    }
}
