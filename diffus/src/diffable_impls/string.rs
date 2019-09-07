use crate::{
    Diffable,
    Edit,
};


impl<'a> Diffable<'a> for String {
    type D = Vec<Edit<'a, String>>;

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
        use difference;

        let (edit_distance, value_diffs) = difference::diff(self, other, "");
        let value_diffs = value_diffs.iter().map(|value_diff| {
            match value_diff {
                difference::Difference::Same(_) => Edit::Copy,
                difference::Difference::Rem(_) => Edit::Remove,
                difference::Difference::Add(a) => Edit::Insert(a),
            }
        });

        if edit_distance > 0 {
            Edit::Change(value_diffs)
        } else {
            Edit::Copy
        }
    }
}
