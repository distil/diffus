
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
    Copy(T, T),
    Insert(T),
    Remove(T),
}

pub(crate) type LcsResult<T> = std::collections::vec_deque::VecDeque<T>;

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
) -> Option<LcsResult<Edit<T>>> {
    let queue = std::collections::VecDeque::with_capacity(c.width + c.height());
    let (queue, modified) = std::iter::from_fn(move || {
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
                (Some(current_x), Some(current_y)) => Some((Edit::Copy(current_x, current_y), false)),
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
                queue,
                false,
            ),
            |(mut queue, modified), (edit, edit_modified)| {
                queue.push_front(edit);
                (queue, modified || edit_modified)
            },
        );
    if modified {
        Some(queue)
    } else {
        None
    }
}

/// Returns the iterator of changes along with a bool indicating if there were any `Insert`/ `Remove`.
pub(crate) fn lcs<T: Same>(
    c: TwoDVec<usize>,
    x: impl DoubleEndedIterator<Item = T>,
    y: impl DoubleEndedIterator<Item = T>,
) -> Option<LcsResult<Edit<T>>> {
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
            Edit::Copy(left, right) => match left.diff(right) {
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
) -> Option<LcsResult<super::edit::collection::Edit<'a, T, <T as Diffable<'a>>::Diff>>> {
    let i = c.width - 1;
    let j = c.height() - 1;
    lcs_base(
        c,
        itertools::put_back(x.rev()),
        itertools::put_back(y.rev()),
        i,
        j,
    )
        .map(|result| enriched_lcs_base(result.into_iter()).collect())
}

/// Same as above but for iterators that don't implement `DoubleEndedIterator`.
/// This means we'll iterate backwards. But collections like `HashSet` doesn't have
/// a concept of direction anyway.
pub(crate) fn enriched_lcs_unordered<'a, T: Same + Diffable<'a> + ?Sized + 'a>(
    c: TwoDVec<usize>,
    x: impl Iterator<Item = &'a T> + 'a,
    y: impl Iterator<Item = &'a T> + 'a,
) -> Option<LcsResult<super::edit::collection::Edit<'a, T, <T as Diffable<'a>>::Diff>>> {
    let i = c.width - 1;
    let j = c.height() - 1;
    lcs_base(
        c,
        itertools::put_back(x),
        itertools::put_back(y),
        i,
        j,
    )
        .map(|result| enriched_lcs_base(result.into_iter()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn characters() {
        let left = "XMJYAUZ";
        let right = "MZJAWXU";

        let c = c_matrix(left.chars(), || right.chars(), left.chars().count(), right.chars().count());
        let s = lcs(c, left.chars(), right.chars()).unwrap();

        assert_eq!(
            s.into_iter().collect::<Vec<_>>(),
            vec![
                Edit::Remove('X'),
                Edit::Copy('M', 'M'),
                Edit::Insert('Z'),
                Edit::Copy('J', 'J'),
                Edit::Remove('Y'),
                Edit::Copy('A', 'A'),
                Edit::Insert('W'),
                Edit::Insert('X'),
                Edit::Copy('U', 'U'),
                Edit::Remove('Z')
            ]
        );
    }

    #[test]
    fn words() {
        let left = "The quick brown fox jumps over the lazy dog";
        let right = "The quick brown dog leaps over the lazy cat";

        let c = c_matrix(left.split_whitespace(), || right.split_whitespace(), left.split_whitespace().count(), right.split_whitespace().count());
        let s = lcs(c, left.split_whitespace(), right.split_whitespace()).unwrap();

        assert_eq!(
            s.into_iter().collect::<Vec<_>>(),
            vec![
                Edit::Copy("The", "The"),
                Edit::Copy("quick", "quick"),
                Edit::Copy("brown", "brown"),
                Edit::Remove("fox"),
                Edit::Remove("jumps"),
                Edit::Insert("dog"),
                Edit::Insert("leaps"),
                Edit::Copy("over", "over"),
                Edit::Copy("the", "the"),
                Edit::Copy("lazy", "lazy"),
                Edit::Remove("dog"),
                Edit::Insert("cat")
            ]
        );
    }
}
