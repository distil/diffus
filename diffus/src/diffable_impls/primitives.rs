use crate::{edit::Edit, Diffable};

macro_rules! for_primitives {
    ($($typ:ty),*) => {
        $(
            impl<'a> Diffable<'a> for $typ {
                type D = (&'a $typ, &'a $typ);

                fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
                    if self == other {
                        Edit::Copy(self)
                    } else {
                        Edit::Change((self, other))
                    }
                }
            }
        )*
    }
}

for_primitives! { i64, i32, i16, i8, u64, u32, u16, u8, char }
