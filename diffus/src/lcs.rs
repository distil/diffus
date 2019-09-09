use super::edit::collection::Edit;

pub(crate) struct Lcs<T: Eq> {
    storage: Vec<usize>,
    width: usize,
    height: usize,
    marker: std::marker::PhantomData<T>,
}

impl<T: Eq> Lcs<T> {
    pub(crate) fn new<I: Iterator<Item = T>>(
        x: impl Iterator<Item = T>,
        y: impl Fn() -> I,
        x_len: usize,
        y_len: usize,
    ) -> Self {
        let width = x_len + 1;
        let height = y_len + 1;

        let mut storage = vec![0; width * height];

        for (i, x) in x.enumerate() {
            for (j, y) in y().enumerate() {
                storage[(i + 1) * width + (j + 1)] = if x == y {
                    storage[i * width + j] + 1
                } else {
                    storage[(i + 1) * width + j].max(storage[i * width + (j + 1)])
                };
            }
        }

        Self {
            storage,
            width,
            height,
            marker: std::marker::PhantomData,
        }
    }

    fn recursive<'a>(
        &self,
        mut x: itertools::PutBack<impl Iterator<Item = T>>,
        mut y: itertools::PutBack<impl Iterator<Item = T>>,
        i: usize,
        j: usize,
    ) -> (Box<dyn Iterator<Item = Edit<T>> + 'a>, bool)
    where
        T: 'a,
    {
        let current_x = x.next();
        let current_y = y.next();

        let left = j
            .checked_sub(1)
            .map(|j_minus| self.storage[i * self.width + j_minus]);
        let above = i
            .checked_sub(1)
            .map(|i_minus| self.storage[i_minus * self.width + j]);

        if current_x.is_some() && current_y.is_some() && current_x == current_y {
            let (recursive, modified) = self.recursive(x, y, i - 1, j - 1);
            (
                Box::new(recursive.chain(current_x.into_iter().map(Edit::Copy))),
                modified,
            )
        } else if current_y.is_some() && (current_x.is_none() || left >= above) {
            current_x.map(|c| x.put_back(c));
            (
                Box::new(
                    self.recursive(x, y, i, j - 1)
                        .0
                        .chain(current_y.into_iter().map(Edit::Insert)),
                ),
                true,
            )
        } else if current_x.is_some() && (current_y.is_none() || left < above) {
            current_y.map(|c| y.put_back(c));
            (
                Box::new(
                    self.recursive(x, y, i - 1, j)
                        .0
                        .chain(current_x.into_iter().map(Edit::Remove)),
                ),
                true,
            )
        } else {
            (Box::new(std::iter::empty()), false)
        }
    }

    /// Returns the iterator of changes along with a bool indicating if there were any `Insert`/ `Remove`.
    pub(crate) fn diff<'a>(
        &self,
        x: impl DoubleEndedIterator<Item = T>,
        y: impl DoubleEndedIterator<Item = T>,
    ) -> (Box<dyn Iterator<Item = Edit<T>> + 'a>, bool)
    where
        T: 'a,
    {
        self.recursive(
            itertools::put_back(x.rev()),
            itertools::put_back(y.rev()),
            self.width - 1,
            self.height - 1,
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn characters() {
        let left = "XMJYAUZ";
        let right = "MZJAWXU";

        let (s, modified) = super::Lcs::new(
            left.chars(),
            || right.chars(),
            left.chars().count(),
            right.chars().count(),
        )
        .diff(left.chars(), right.chars());
        assert!(modified);
        use super::Edit::*;
        assert_eq!(
            s.collect::<Vec<_>>(),
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
    }

    #[test]
    fn words() {
        let left = "The quick brown fox jumps over the lazy dog";
        let right = "The quick brown dog leaps over the lazy cat";

        let (s, modified) = super::Lcs::new(
            left.split_whitespace(),
            || right.split_whitespace(),
            left.split_whitespace().count(),
            right.split_whitespace().count(),
        )
        .diff(left.split_whitespace(), right.split_whitespace());
        assert!(modified);
        use super::Edit::*;
        assert_eq!(
            s.collect::<Vec<_>>(),
            vec![
                Copy("The"),
                Copy("quick"),
                Copy("brown"),
                Remove("fox"),
                Remove("jumps"),
                Insert("dog"),
                Insert("leaps"),
                Copy("over"),
                Copy("the"),
                Copy("lazy"),
                Remove("dog"),
                Insert("cat")
            ]
        );
    }
}
