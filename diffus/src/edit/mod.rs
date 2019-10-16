pub mod collection;
pub mod enm;
pub mod map;

#[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
#[derive(Debug, PartialEq)]
pub enum Edit<Diff> {
    Copy,
    Change(Diff),
}

impl<Diff> Edit<Diff> {
    pub fn is_copy(&self) -> bool {
        if let Self::Copy = self {
            true
        } else {
            false
        }
    }

    pub fn is_change(&self) -> bool {
        if let Self::Change(_) = self {
            true
        } else {
            false
        }
    }

    pub fn change(&self) -> Option<&Diff> {
        if let Self::Change(value_diff) = self {
            Some(value_diff)
        } else {
            None
        }
    }
}

impl<'a, Diff, T: crate::Diffable<'a, Diff = Diff> + 'a> Into<map::Edit<'a, T>> for Edit<Diff> {
    fn into(self) -> map::Edit<'a, T> {
        match self {
            Self::Copy => map::Edit::Copy,
            Self::Change(diff) => map::Edit::Change(diff),
        }
    }
}
