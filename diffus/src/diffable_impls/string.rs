use crate::Diffable;

pub type Edit = crate::lcs::Edit<char>;

pub struct Diff(pub(crate) crate::lcs::LcsResult<Edit>);

pub struct IntoIter(<crate::lcs::LcsResult<Edit> as std::iter::IntoIterator>::IntoIter);

impl std::iter::IntoIterator for Diff {
    type Item = Edit;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

impl std::iter::Iterator for IntoIter {
    type Item = Edit;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> Diffable<'a> for str {
    type Diff = Diff;

    fn diff(&'a self, other: &'a Self) -> crate::edit::Edit<Self::Diff> {
        let c = crate::lcs::c_matrix(self.chars(), || other.chars(), self.chars().count(), other.chars().count());
        if let Some(s) = crate::lcs::lcs(c, self.chars(), other.chars()) {
            crate::edit::Edit::Change(Diff(s))
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
