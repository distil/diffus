use crate::{
    Diffable,
};

pub type Edit = crate::lcs::Edit<char>;

impl<'a> Diffable<'a> for str {
    type Diff = super::collection::CollectionDiff<Edit>;

    fn diff(&'a self, other: &'a Self) -> crate::edit::Edit<Self::Diff> {
        let c = crate::lcs::c_matrix(self.chars(), || other.chars(), self.chars().count(), other.chars().count());
        if let Some(s) = crate::lcs::lcs(c, self.chars(), other.chars()) {
            crate::edit::Edit::Change(super::collection::CollectionDiff(s))
        } else {
            crate::edit::Edit::Copy
        }
    }
}

impl<'a> Diffable<'a> for String {
    type Diff = <str as Diffable<'a>>::Diff;

    fn diff(&'a self, other: &'a Self) -> crate::edit::Edit<Self::Diff> {
        self.as_str().diff(other.as_str())
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
        if let crate::edit::Edit::Change(diff) = diff {
            assert_eq!(
                diff.into_iter().collect::<Vec<_>>(),
                vec![
                    Edit::Remove('X'),
                    Edit::Copy('M', 'M'),
                    Edit::Insert('Z'),
                    Edit::Copy('J', 'J'),
                    Edit::Remove('Y'),
                    Edit::Copy('A', 'A'),
                    Edit::Insert('W'),
                    Edit::Insert('X'),
                    Edit::Copy('U', 'U'),
                    Edit::Remove('Z')
                ]
            );
        } else {
            unreachable!()
        }
    }
}
