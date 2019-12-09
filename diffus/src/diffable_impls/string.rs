use crate::{
    edit::{self, string},
    lcs, Diffable,
};

impl<'a> Diffable<'a> for str {
    type Diff = Vec<string::Edit>;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<Self> {
        let s = lcs::lcs(
            || self.chars(),
            || other.chars(),
            self.chars().count(),
            other.chars().count(),
        )
        .map(Into::into)
        .collect::<Vec<_>>();

        if s.iter().all(string::Edit::is_copy) {
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
    use crate::edit::{self, string};

    #[test]
    fn string() {
        use super::Diffable;

        let left = "XMJYAUZ".to_owned();
        let right = "MZJAWXU".to_owned();

        let diff = left.diff(&right);
        if let edit::Edit::Change(diff) = diff {
            assert_eq!(
                diff.into_iter().collect::<Vec<_>>(),
                vec![
                    string::Edit::Remove('X'),
                    string::Edit::Copy('M'),
                    string::Edit::Insert('Z'),
                    string::Edit::Copy('J'),
                    string::Edit::Remove('Y'),
                    string::Edit::Copy('A'),
                    string::Edit::Insert('W'),
                    string::Edit::Insert('X'),
                    string::Edit::Copy('U'),
                    string::Edit::Remove('Z')
                ]
            );
        } else {
            unreachable!()
        }
    }

    #[test]
    fn str() {
        use super::Diffable;

        let left = "XMJYAUZ";
        let right = "MZJAWXU";

        let diff = left.diff(&right);
        if let edit::Edit::Change(diff) = diff {
            assert_eq!(
                diff.into_iter().collect::<Vec<_>>(),
                vec![
                    string::Edit::Remove('X'),
                    string::Edit::Copy('M'),
                    string::Edit::Insert('Z'),
                    string::Edit::Copy('J'),
                    string::Edit::Remove('Y'),
                    string::Edit::Copy('A'),
                    string::Edit::Insert('W'),
                    string::Edit::Insert('X'),
                    string::Edit::Copy('U'),
                    string::Edit::Remove('Z')
                ]
            );
        } else {
            unreachable!()
        }
    }
}
