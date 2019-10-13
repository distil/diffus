use crate::{
    edit::{collection, Edit},
    lcs::Lcs,
    Diffable,
};

/* FIXME
impl<'a> Diffable<'a> for str {
    type Diff = Vec<collection::Edit<char, (char, char)>>;

    fn diff(&'a self, other: &'a Self) -> Edit<Self::Diff> {
        let self_chars = self.chars().collect::<Vec<_>>();
        let other_chars = other.chars().collect::<Vec<_>>();
        let (s, modified) = Lcs::new(
            self_chars.iter(),
            || other_chars.iter(),
            self_chars.len(),
            other_chars.len(),
        )
        .diff(self_chars.iter(), other_chars.iter());

        let s = s.iter()
            .map(|edit| match edit {
                collection::Edit::Remove(ch) => collection::Edit::Remove(*ch),
                collection::Edit::Insert(ch) => collection::Edit::Insert(*ch),
                collection::Edit::Copy(ch) => collection::Edit::Copy(*ch),
                collection::Edit::Change((left, right)) => {
                    collection::Edit::Change((*left, *right))
                }
            })
            .collect();
        if modified {
            Edit::Change(s)
        } else {
            Edit::Copy
        }
    }
}

impl<'a> Diffable<'a> for String {
    type Diff = <str as Diffable<'a>>::Diff;

    fn diff(&'a self, other: &'a Self) -> Edit<Self::Diff> {
        self.as_str().diff(other.as_str())
    }
}


#[cfg(test)]
mod tests {
    use super::{collection::Edit::*, *};

    #[test]
    fn diff() {
        use super::Diffable;

        let left = "XMJYAUZ".to_owned();
        let right = "MZJAWXU".to_owned();

        let diff = left.diff(&right);
        if let Edit::Change(diff) = diff {
            assert_eq!(
                diff,
                vec![
                    Remove('X'),
                    Copy('M'),
                    Insert('Z'),
                    Copy('J'),
                    Remove('Y'),
                    Copy('A'),
                    Insert('W'),
                    Insert('X'),
                    Copy('U'),
                    Remove('Z')
                ]
            );
        } else {
            unreachable!()
        }
    }
}

*/
