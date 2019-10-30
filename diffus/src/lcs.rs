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

fn c_matrix<T: Same, I: Iterator<Item = T>, J: Iterator<Item = T>>(
    x: impl Fn() -> I,
    y: impl Fn() -> J,
    x_len: usize,
    y_len: usize,
) -> crate::twodvec::TwoDVec<usize> {
    let width = x_len + 1;
    let height = y_len + 1;

    let mut c = crate::twodvec::TwoDVec::new(0, width, height);

    for (i, x) in x().enumerate() {
        for (j, y) in y().enumerate() {
            c[j + 1][i + 1] = if x.same(&y) {
                c[j][i] + 1
            } else {
                c[j][i + 1].max(c[j + 1][i])
            };
        }
    }

    c
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
    lcs_base(
        c_matrix(|| x(), || y(), x_len, y_len),
        itertools::put_back(x().rev()),
        itertools::put_back(y().rev()),
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
