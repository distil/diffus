use crate::{edit, Diffable, Same};

#[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
#[derive(Debug, PartialEq, Eq)]
pub enum Edit<T> {
    Same(T, T),
    Insert(T),
    Remove(T),
}

impl<T> Edit<T> {
    pub fn is_same(&self) -> bool {
        if let Edit::Same(_, _) = self {
            true
        } else {
            false
        }
    }
}

fn c_matrix<T: Same, I, J>(
    x: impl Fn() -> I,
    y: impl Fn() -> J,
    x_len: usize,
    y_len: usize,
) -> (usize, crate::twodvec::TwoDVec<usize>, usize)
where
    I: DoubleEndedIterator<Item = T>,
    J: DoubleEndedIterator<Item = T>,
{
    let mut prefix_eq = 0;
    let mut suffix_eq = 0;
    {
        let mut x_iter = x();
        let mut y_iter = y();
        // We must not compute an overlapping `prefix_eq` and `suffix_eq` so we only check the suffix
        // if there is something that is not the same in the middle of the iterators
        let mut check_suffix = false;
        loop {
            match (x_iter.next(), y_iter.next()) {
                (Some(x), Some(y)) if x.same(&y) => prefix_eq += 1,
                (Some(_), Some(_)) => {
                    check_suffix = true;
                    break;
                }
                (None, _) | (_, None) => break,
            }
        }
        if check_suffix {
            suffix_eq = x_iter
                .rev()
                .zip(y_iter.rev())
                .take_while(|(x, y)| x.same(y))
                .count();
        }
    }

    let width = x_len.saturating_sub(prefix_eq + suffix_eq) + 1;
    let height = y_len.saturating_sub(prefix_eq + suffix_eq) + 1;

    let mut c = crate::twodvec::TwoDVec::new(0, width, height);

    for (i, x) in x().skip(prefix_eq).take(width - 1).enumerate() {
        for (j, y) in y().skip(prefix_eq).take(height - 1).enumerate() {
            c[j + 1][i + 1] = if x.same(&y) {
                c[j][i] + 1
            } else {
                c[j][i + 1].max(c[j + 1][i])
            };
        }
    }

    (prefix_eq, c, suffix_eq)
}

fn lcs_base<T: Same>(
    c: crate::twodvec::TwoDVec<usize>,
    mut x: itertools::PutBack<impl Iterator<Item = T>>,
    mut y: itertools::PutBack<impl Iterator<Item = T>>,
) -> impl Iterator<Item = Edit<T>> {
    let mut i = c.width() - 1;
    let mut j = c.height() - 1;

    std::iter::from_fn(move || {
        let current_x = x.next();
        let current_y = y.next();

        let left = j.checked_sub(1).map(|j_minus| c[j_minus][i]);
        let above = i.checked_sub(1).map(|i_minus| c[j][i_minus]);

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
                (Some(current_x), Some(current_y)) => Some(Edit::Same(current_x, current_y)),
                _ => unreachable!(),
            }
        } else if current_y.is_some() && (current_x.is_none() || left >= above) {
            current_x.map(|c| x.put_back(c));
            j = j - 1;
            current_y.map(|value| Edit::Insert(value))
        } else if current_x.is_some() && (current_y.is_none() || left < above) {
            current_y.map(|c| y.put_back(c));
            i = i - 1;
            current_x.map(|value| Edit::Remove(value))
        } else {
            None
        }
    })
    .collect::<Vec<_>>()
    .into_iter()
    .rev()
}

pub(crate) fn lcs<
    'a,
    T: Same,
    I: DoubleEndedIterator<Item = T>,
    J: DoubleEndedIterator<Item = T>,
>(
    x: impl Fn() -> I,
    y: impl Fn() -> J,
    x_len: usize,
    y_len: usize,
) -> impl Iterator<Item = Edit<T>> {
    let (prefix_eq, c, suffix_eq) = c_matrix(&x, &y, x_len, y_len);

    x().zip(y())
        .take(prefix_eq)
        .map(|(x, y)| Edit::Same(x, y))
        .chain(lcs_base(
            c,
            itertools::put_back(
                x().rev()
                    .skip(suffix_eq)
                    .take(x_len.saturating_sub(prefix_eq + suffix_eq)),
            ),
            itertools::put_back(
                y().rev()
                    .skip(suffix_eq)
                    .take(y_len.saturating_sub(prefix_eq + suffix_eq)),
            ),
        ))
        .chain(
            x().skip(x_len - suffix_eq)
                .zip(y().skip(y_len - suffix_eq))
                .map(|(x, y)| Edit::Same(x, y)),
        )
}

// FIXME move out from lcs
pub(crate) fn lcs_post_change<'a, T: Same + Diffable<'a> + ?Sized + 'a>(
    result: impl Iterator<Item = Edit<&'a T>>,
) -> impl Iterator<Item = edit::collection::Edit<'a, T, <T as Diffable<'a>>::Diff>> {
    result.map(|edit| match edit {
        Edit::Same(left, right) => match left.diff(right) {
            edit::Edit::Copy(t) => edit::collection::Edit::Copy(t),
            edit::Edit::Change(diff) => edit::collection::Edit::Change(diff),
        },
        Edit::Insert(value) => edit::collection::Edit::Insert(value),
        Edit::Remove(value) => edit::collection::Edit::Remove(value),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn characters() {
        let left = "XMJYAUZ";
        let right = "MZJAWXU";

        let s = lcs(
            || left.chars(),
            || right.chars(),
            left.chars().count(),
            right.chars().count(),
        );

        assert_eq!(
            s.collect::<Vec<_>>(),
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
    }

    #[test]
    fn words() {
        let left = "The quick brown fox jumps over the lazy dog";
        let right = "The quick brown dog leaps over the lazy cat";

        let s = lcs(
            || left.split_whitespace(),
            || right.split_whitespace(),
            left.split_whitespace().count(),
            right.split_whitespace().count(),
        );

        assert_eq!(
            s.collect::<Vec<_>>(),
            vec![
                Edit::Same("The", "The"),
                Edit::Same("quick", "quick"),
                Edit::Same("brown", "brown"),
                Edit::Remove("fox"),
                Edit::Remove("jumps"),
                Edit::Insert("dog"),
                Edit::Insert("leaps"),
                Edit::Same("over", "over"),
                Edit::Same("the", "the"),
                Edit::Same("lazy", "lazy"),
                Edit::Remove("dog"),
                Edit::Insert("cat")
            ]
        );
    }
}
