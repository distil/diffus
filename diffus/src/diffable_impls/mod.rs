pub mod map;
pub mod primitives;
pub mod string;
pub mod collection;
pub mod option;

impl<'a, T: crate::Diffable<'a> + 'a> crate::Diffable<'a> for &'a T {
    type D = T::D;
    type Target = &'a Self;

    fn diff(&'a self, other: &'a Self) -> crate::edit::Edit<'a, Self::Target> {
        (*self).diff(*other)
    }
}
