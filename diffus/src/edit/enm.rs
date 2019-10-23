#[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
#[derive(Debug, Eq, PartialEq)]
pub enum Edit<'a, T: ?Sized, Diff> {
    Copy,
    VariantChanged(&'a T, &'a T),
    AssociatedChanged(Diff),
}

impl<'a, T: ?Sized, D> Edit<'a, T, D> {
    pub fn is_copy(&self) -> bool {
        if let Self::Copy = self {
            true
        } else {
            false
        }
    }

    pub fn is_variant_changed(&self) -> bool {
        if let Self::VariantChanged(_, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_associated_changed(&self) -> bool {
        if let Self::AssociatedChanged(_) = self {
            true
        } else {
            false
        }
    }

    pub fn variant_changed(&self) -> Option<(&'a T, &'a T)> {
        if let Self::VariantChanged(left, right) = self {
            Some((left, right))
        } else {
            None
        }
    }

    pub fn associated_change(&self) -> Option<&D> {
        if let Self::AssociatedChanged(value) = self {
            Some(value)
        } else {
            None
        }
    }
}
