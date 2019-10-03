pub mod map;
pub mod primitives;
pub mod string;
pub mod collection;
pub mod option;

impl<'a, T: crate::Diffable<'a> + 'a> crate::Diffable<'a> for &'a T
{
    type D = T::D;
    type Target = T::Target;

    fn diff(&self, other: &Self) -> crate::edit::Edit<'a, Self::Target> {
        (*self).diff(*other)
    }
}
