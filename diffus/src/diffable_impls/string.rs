use crate::{
    edit::{collection, Edit},
    lcs::Lcs,
    Diffable,
};


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
        .diff(
            self_chars.iter(),
            other_chars.iter(),
        );

        let s = {
            use collection::Edit::*;

            s.iter()
                .map(|edit| match edit {
                    Remove(ch) => Remove(**ch),
                    Insert(ch) => Insert(**ch),
                    Copy(ch) => Copy(**ch),
                    Change((left, right)) => Change((**left, **right)),
                })
        }
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
