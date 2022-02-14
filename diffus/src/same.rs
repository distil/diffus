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

same_for_eq! { i64, i32, i16, i8, u64, u32, u16, u8, char, str, String, bool, isize, usize, () }

macro_rules! same_for_float {
    ($($typ:ty),*) => {
        $(
            impl Same for $typ {
                fn same(&self, other: &Self) -> bool {
                    self.to_ne_bytes() == other.to_ne_bytes()
                }
            }
        )*
    }
}

same_for_float! { f32, f64 }

#[cfg(feature = "snake_case-impl")]
same_for_eq! { snake_case::SnakeCase }

#[cfg(feature = "uuid-impl")]
same_for_eq! { uuid::Uuid }

impl<T: Same + ?Sized> Same for &T {
    fn same(&self, other: &Self) -> bool {
        (*self).same(*other)
    }
}
