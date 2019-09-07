use crate::{
    Diffable,
    Edit,
};


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
