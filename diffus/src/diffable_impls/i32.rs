use crate::{
    Diffable,
    Edit,
};

impl<'a> Diffable<'a> for i32 {
    type D = (&'a i32, &'a i32);

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
        if self == other {
            Edit::Copy(self)
        } else {
            Edit::Change((self, other))
        }
    }
}
