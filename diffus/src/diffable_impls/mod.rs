pub mod collection;
pub mod map;
pub mod option;
pub mod primitives;
pub mod string;

impl<'a, T: crate::Diffable<'a> + 'a> crate::Diffable<'a> for &'a T {
    type Diff = T::Diff;

    fn diff(&self, other: &Self) -> crate::edit::Edit<Self::Diff> {
        (*self).diff(*other)
    }
}
