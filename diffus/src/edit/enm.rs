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

    pub fn is_change(&self) -> bool {
        !self.is_copy()
    }
}
