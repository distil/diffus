use crate::{Diffable, Edit, EditSection, lcs::Lcs};

impl<'a, T: Eq + 'a> Diffable<'a> for Vec<T> {
    type D = Box<dyn Iterator<Item = EditSection<&'a T>> + 'a>;

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
        let (s, modified) = Lcs::new(
            self.iter(),
            || other.iter(),
            self.iter().count(),
            other.iter().count(),
        )
            .diff(self.iter(), other.iter());

        if modified {
            Edit::Change(s)
        } else {
            Edit::Copy(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{*, EditSection::*};

    #[test]
    fn diff() {
        use super::Diffable;

        let left = b"XMJYAUZ".to_vec();
        let right = b"MZJAWXU".to_vec();

        let diff = left.diff(&right);
        if let Edit::Change(diff) = diff {
            assert_eq!(
                diff.collect::<Vec<_>>(),
                vec![
                    Remove(&b'X'),
                    Copy(&b'M'),
                    Add(&b'Z'),
                    Copy(&b'J'),
                    Remove(&b'Y'),
                    Copy(&b'A'),
                    Add(&b'W'),
                    Add(&b'X'),
                    Copy(&b'U'),
                    Remove(&b'Z')
                ]);
        } else {
            unreachable!()
        }
    }
}
