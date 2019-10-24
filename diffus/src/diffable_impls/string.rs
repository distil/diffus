use crate::Diffable;

pub type Edit = crate::lcs::Edit<char>;

impl<'a> Diffable<'a> for str {
    type Diff = Vec<Edit>;

    fn diff(&'a self, other: &'a Self) -> crate::edit::Edit<Self::Diff> {
        let c = crate::lcs::c_matrix(self.chars(), || other.chars(), self.chars().count(), other.chars().count());
        let s = crate::lcs::lcs(c, self.chars(), other.chars()).collect::<Vec<_>>();
        if s.iter().all(Edit::is_same) {
            crate::edit::Edit::Copy
        } else {
            crate::edit::Edit::Change(s)
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
                    Edit::Same('M', 'M'),
                    Edit::Insert('Z'),
                    Edit::Same('J', 'J'),
                    Edit::Remove('Y'),
                    Edit::Same('A', 'A'),
                    Edit::Insert('W'),
                    Edit::Insert('X'),
                    Edit::Same('U', 'U'),
                    Edit::Remove('Z')
                ]
            );
        } else {
            unreachable!()
        }
    }
}
