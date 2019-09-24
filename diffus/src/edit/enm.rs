// TODO figure out the reason why it's not Edit<T> { .., AssociatedChanged(T::D) }
pub enum Edit<'a, T, D> {
    Copy,
    VariantChanged((&'a T, &'a T)),
    AssociatedChanged(D),
}

impl<'a, T, D> Edit<'a, T, D> {
    pub fn is_copy(&self) -> bool {
        if let Self::Copy = self {
            true
        } else {
            false
        }
    }
}
