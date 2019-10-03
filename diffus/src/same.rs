use crate::Same;

impl<T: Same> Same for Option<T> {
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => a.same(b),
            (None, None) => true,
            _ => false,
        }
    }
}

macro_rules! same_for_eq {
    ($($typ:ty),*) => {
        $(
            impl Same for $typ {
                fn same(&self, other: &Self) -> bool {
                    self == other
                }
            }
        )*
    }
}

same_for_eq! { u8, u16, u32, u64, i8, i16, i32, i64, char, str, String }

#[cfg(feature = "snake_case-impl")]
same_for_eq! { snake_case::SnakeCase }

#[cfg(feature = "uuid-impl")]
same_for_eq! { uuid::Uuid }

impl<T: Same> Same for &T {
    fn same(&self, other: &Self) -> bool {
        (*self).same(*other)
    }
}
