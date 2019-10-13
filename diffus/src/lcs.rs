use super::edit::collection::Edit;

use crate::{Diffable, Same};

#[cfg(feature = "serialize-impl")]
use serde::Serialize;


macro_rules! lcs {
    ( : $($constraint:ident),* ) => {
        pub(crate) struct Lcs<T: Same + ?Sized> {
            storage: Vec<usize>,
            width: usize,
            height: usize,
            marker: std::marker::PhantomData<T>,
        }

        impl<'a, T: Same $(+$constraint)? + Diffable<'a> + ?Sized + 'a> Lcs<T> {
            pub(crate) fn new<I: Iterator<Item = &'a T>>(
                x: impl Iterator<Item = &'a T>,
                y: impl Fn() -> I,
                x_len: usize,
                y_len: usize,
            ) -> Self {
                let width = x_len + 1;
                let height = y_len + 1;

                let mut storage = vec![0; width * height];

                let a = |i: usize, j: usize| {
                    let result = j * width + i;

                    result
                };

                for (i, x) in x.enumerate() {
                    for (j, y) in y().enumerate() {
                        storage[a(i + 1, j + 1)] = if x.same(&y) {
                            storage[a(i, j)] + 1
                        } else {
                            storage[a(i + 1, j)].max(storage[a(i, j + 1)])
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

            fn diff_impl(
                &self,
                mut x: itertools::PutBack<impl Iterator<Item = &'a T>>,
                mut y: itertools::PutBack<impl Iterator<Item = &'a T>>,
                mut i: usize,
                mut j: usize,
            ) -> (
                Vec<Edit<&'a T, <T as Diffable<'a>>::Diff>>,
                bool,
            )
            where
                T: 'a,
            {
                let a = |i, j| j * self.width + i;

                let (queue, modified) = std::iter::from_fn(move || {
                    let current_x = x.next();
                    let current_y = y.next();

                    let left = j.checked_sub(1).map(|j_minus| self.storage[a(i, j_minus)]);
                    let above = i.checked_sub(1).map(|i_minus| self.storage[a(i_minus, j)]);

                    if current_x.is_some()
                        && current_y.is_some()
                        && current_x
                            .as_ref()
                            .unwrap()
                            .same(current_y.as_ref().unwrap())
                    {
                        i = i - 1;
                        j = j - 1;

                        match (current_x, current_y) {
                            (Some(current_x), Some(current_y)) => match current_x.diff(&current_y) {
                                crate::edit::Edit::Copy => Some((Edit::Copy(current_x), false)),
                                crate::edit::Edit::Change(diff) => Some((Edit::Change(diff), true)),
                            },
                            _ => unreachable!(),
                        }
                    } else if current_y.is_some() && (current_x.is_none() || left >= above) {
                        current_x.map(|c| x.put_back(c));
                        j = j - 1;
                        current_y.map(|value| (Edit::Insert(value), true))
                    } else if current_x.is_some() && (current_y.is_none() || left < above) {
                        current_y.map(|c| y.put_back(c));
                        i = i - 1;
                        current_x.map(|value| (Edit::Remove(value), true))
                    } else {
                        None
                    }
                })
                .fold(
                    (
                        std::collections::VecDeque::with_capacity(self.width + self.height),
                        false,
                    ),
                    |(mut queue, modified), (edit, edit_modified)| {
                        queue.push_front(edit);
                        (queue, modified || edit_modified)
                    },
                );

                (queue.into_iter().collect::<Vec<_>>(), modified)
            }

            /// Returns the iterator of changes along with a bool indicating if there were any `Insert`/ `Remove`.
            pub(crate) fn diff(
                &self,
                x: impl DoubleEndedIterator<Item = &'a T> + 'a,
                y: impl DoubleEndedIterator<Item = &'a T> + 'a,
            ) -> (
                Vec<Edit<&'a T, <T as Diffable<'a>>::Diff>>,
                bool,
            )
            where
                T: 'a,
            {
                self.diff_impl(
                    itertools::put_back(x.rev()),
                    itertools::put_back(y.rev()),
                    self.width - 1,
                    self.height - 1,
                )
            }

            /// Same as above but for iterators that don't implement `DoubleEndedIterator`.
            /// This means we'll iterate backwards. But collections like `HashSet` doesn't have
            /// a concept of direction anyway.
            pub(crate) fn diff_unordered(
                &self,
                x: impl Iterator<Item = &'a T> + 'a,
                y: impl Iterator<Item = &'a T> + 'a,
            ) -> (
                Vec<Edit<&'a T, <T as Diffable<'a>>::Diff>>,
                bool,
            )
            where
                T: 'a,
            {
                self.diff_impl(
                    itertools::put_back(x),
                    itertools::put_back(y),
                    self.width - 1,
                    self.height - 1,
                )
            }
        }
    }
}

#[cfg(feature = "serialize-impl")]
lcs!{ : Serialize }
#[cfg(not(feature = "serialize-impl"))]
lcs!{ : }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn characters() {
        let left = "XMJYAUZ";
        let right = "MZJAWXU";
        let left_chars = left.chars().collect::<Vec<_>>();
        let right_chars = right.chars().collect::<Vec<_>>();

        let (s, modified) = Lcs::new(
            left_chars.iter(),
            || right_chars.iter(),
            left_chars.len(),
            right_chars.len(),
        )
        .diff(left_chars.iter(), right_chars.iter());
        assert!(modified);
        use Edit::*;
        assert_eq!(
            s.collect::<Vec<_>>(),
            vec![
                Remove(&'X'),
                Copy(&'M'),
                Insert(&'Z'),
                Copy(&'J'),
                Remove(&'Y'),
                Copy(&'A'),
                Insert(&'W'),
                Insert(&'X'),
                Copy(&'U'),
                Remove(&'Z')
            ]
        );
    }

    #[test]
    fn words() {
        let left = "The quick brown fox jumps over the lazy dog";
        let right = "The quick brown dog leaps over the lazy cat";

        let (s, modified) = Lcs::new(
            left.split_whitespace(),
            || right.split_whitespace(),
            left.split_whitespace().count(),
            right.split_whitespace().count(),
        )
        .diff(left.split_whitespace(), right.split_whitespace());
        assert!(modified);
        use Edit::*;
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

