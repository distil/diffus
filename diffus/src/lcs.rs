
use crate::Diffable;
use crate::Same;

pub(crate) struct TwoDVec<T> {
    storage: Vec<T>,
    width: usize,
}

impl<T: Clone> TwoDVec<T> {
    pub fn new(initial: T, width: usize, height: usize) -> Self {
        Self {
            storage: vec![initial; width * height],
            width,
        }
    }
}

impl<T> TwoDVec<T> {
    pub fn height(&self) -> usize {
        self.storage.len() / self.width
    }
}

impl<T> std::ops::Index<usize> for TwoDVec<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.storage.as_slice()[self.width * index..][..self.width]
    }
}

impl<T> std::ops::IndexMut<usize> for TwoDVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.storage.as_mut_slice()[self.width * index..][..self.width]
    }
}

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

pub(crate) fn c_matrix<T: Same, I: Iterator<Item = T>>(
    x: impl Iterator<Item = T>,
    y: impl Fn() -> I,
    x_len: usize,
    y_len: usize,
) -> TwoDVec<usize> {
    let width = x_len + 1;
    let height = y_len + 1;

    let mut c = TwoDVec::new(0, width, height);

    for (i, x) in x.enumerate() {
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
    c: TwoDVec<usize>,
    mut x: itertools::PutBack<impl Iterator<Item = T>>,
    mut y: itertools::PutBack<impl Iterator<Item = T>>,
    mut i: usize,
    mut j: usize,
) -> impl Iterator<Item = Edit<T>> {
    std::iter::from_fn(move || {
        let current_x = x.next();
        let current_y = y.next();

        let left = j
            .checked_sub(1)
            .map(|j_minus| c[j_minus][i]);
        let above = i
            .checked_sub(1)
            .map(|i_minus| c[j][i_minus]);

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

/// Returns the iterator of changes along with a bool indicating if there were any `Insert`/ `Remove`.
pub(crate) fn lcs<T: Same>(
    c: TwoDVec<usize>,
    x: impl DoubleEndedIterator<Item = T>,
    y: impl DoubleEndedIterator<Item = T>,
) -> impl Iterator<Item = Edit<T>> {
    let i = c.width - 1;
    let j = c.height() - 1;
    lcs_base(
        c,
        itertools::put_back(x.rev()),
        itertools::put_back(y.rev()),
        i,
        j,
    )
}

fn enriched_lcs_base<'a, T: Same + Diffable<'a> + ?Sized + 'a>(
    result: impl Iterator<Item = Edit<&'a T>>
) -> impl Iterator<Item = super::edit::collection::Edit<'a, T, <T as Diffable<'a>>::Diff>> {
    result
        .map(|edit| match edit {
            Edit::Same(left, right) => match left.diff(right) {
                super::edit::Edit::Copy => super::edit::collection::Edit::Copy(left),
                super::edit::Edit::Change(diff) => super::edit::collection::Edit::Change(diff),
            },
            Edit::Insert(value) => super::edit::collection::Edit::Insert(value),
            Edit::Remove(value) => super::edit::collection::Edit::Remove(value),
        })
}

/// Returns the iterator of changes along with a bool indicating if there were any `Insert`/ `Remove`.
pub(crate) fn enriched_lcs<'a, T: Same + Diffable<'a> + ?Sized + 'a>(
    c: TwoDVec<usize>,
    x: impl DoubleEndedIterator<Item = &'a T> + 'a,
    y: impl DoubleEndedIterator<Item = &'a T> + 'a,
) -> impl Iterator<Item = super::edit::collection::Edit<'a, T, <T as Diffable<'a>>::Diff>> {
    let i = c.width - 1;
    let j = c.height() - 1;
    enriched_lcs_base(lcs_base(
        c,
        itertools::put_back(x.rev()),
        itertools::put_back(y.rev()),
        i,
        j,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn characters() {
        let left = "XMJYAUZ";
        let right = "MZJAWXU";

        let c = c_matrix(left.chars(), || right.chars(), left.chars().count(), right.chars().count());
        let s = lcs(c, left.chars(), right.chars());

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

        let c = c_matrix(left.split_whitespace(), || right.split_whitespace(), left.split_whitespace().count(), right.split_whitespace().count());
        let s = lcs(c, left.split_whitespace(), right.split_whitespace());

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
