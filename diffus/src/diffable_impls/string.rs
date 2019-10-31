use crate::{edit, lcs, Diffable};

#[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
#[derive(Debug, PartialEq, Eq)]
pub enum Edit {
    Copy(char),
    Insert(char),
    Remove(char),
}

impl From<lcs::Edit<char>> for Edit {
    fn from(edit: lcs::Edit<char>) -> Self {
        use lcs::Edit::*;
        match edit {
            Same(left, _) => Self::Copy(left),
            Insert(value) => Self::Insert(value),
            Remove(value) => Self::Remove(value),
        }
    }
}

impl Edit {
    pub fn is_copy(&self) -> bool {
        if let Self::Copy(_) = self {
            true
        } else {
            false
        }
    }
}

impl<'a> Diffable<'a> for str {
    type Diff = Vec<Edit>;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<Self> {
        let s = lcs::lcs(
            || self.chars(),
            || other.chars(),
            self.chars().count(),
            other.chars().count(),
        )
        .map(Into::into)
        .collect::<Vec<_>>();

        if s.iter().all(Edit::is_copy) {
            edit::Edit::Copy(self)
        } else {
            edit::Edit::Change(s)
        }
    }
}

impl<'a> Diffable<'a> for String {
    type Diff = <str as Diffable<'a>>::Diff;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<Self> {
        match self.as_str().diff(other.as_str()) {
            edit::Edit::Change(diff) => edit::Edit::Change(diff),
            edit::Edit::Copy(_) => edit::Edit::Copy(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff() {
        use super::Diffable;

        let left = "XMJYAUZ".to_owned();
        let right = "MZJAWXU".to_owned();

        let diff = left.diff(&right);
        if let edit::Edit::Change(diff) = diff {
            assert_eq!(
                diff.into_iter().collect::<Vec<_>>(),
                vec![
                    Edit::Remove('X'),
                    Edit::Copy('M'),
                    Edit::Insert('Z'),
                    Edit::Copy('J'),
                    Edit::Remove('Y'),
                    Edit::Copy('A'),
                    Edit::Insert('W'),
                    Edit::Insert('X'),
                    Edit::Copy('U'),
                    Edit::Remove('Z')
                ]
            );
        } else {
            unreachable!()
        }
    }
}
