use crate::{edit::{collection, Edit}, lcs::Lcs, Diffable};

impl<'a> Diffable<'a> for str {
    type D = Vec<collection::Edit<char, (char, char)>>;
    type Target = Self;

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self::Target> {
        let self_chars = self.chars().collect::<Vec<_>>();
        let other_chars = other.chars().collect::<Vec<_>>();
        let (s, modified) = Lcs::new(
            self_chars.iter(),
            || other_chars.iter(),
            self_chars.len(),
            other_chars.len(),
        )
            .diff(self_chars.iter(), other_chars.iter());

        // TODO: The above Lcs only handles iterators to references, but characters are
        // TODO: intermediates. The conversion is done here but ideally should never need to
        // TODO: be done at all
        let s = s
            .map(|edit| match edit {
                collection::Edit::Remove(ch) => collection::Edit::Remove(*ch),
                collection::Edit::Insert(ch) => collection::Edit::Insert(*ch),
                collection::Edit::Copy(ch) => collection::Edit::Copy(*ch),
                collection::Edit::Change((left, right)) => collection::Edit::Change((*left, *right)),
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
    type D = <str as Diffable<'a>>::D;
    type Target = str;

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self::Target> {
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
