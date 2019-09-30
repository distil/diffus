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

// FIXME impl primitives
impl Same for char {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Same for u8 {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Same for &str {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl<T: Same> Same for &T {
    fn same(&self, other: &Self) -> bool {
        (*self).same(*other)
    }
}
