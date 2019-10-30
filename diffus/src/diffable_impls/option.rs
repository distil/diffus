use crate::{
    edit::{self, enm},
    Diffable,
};

impl<'a, T: Diffable<'a> + 'a> Diffable<'a> for Option<T> {
    type Diff = enm::Edit<'a, Self, T::Diff>;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<Self> {
        match (self, other) {
            (None, None) => edit::Edit::Copy(self),
            (Some(a), Some(b)) => match a.diff(&b) {
                edit::Edit::Copy(_) => edit::Edit::Copy(self),
                edit::Edit::Change(diff) => edit::Edit::Change(enm::Edit::AssociatedChanged(diff)),
            },
            _ => edit::Edit::Change(enm::Edit::VariantChanged(self, other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_copy() {
        assert!((None as Option<u32>).diff(&None).is_copy());
        assert!(Some(3).diff(&Some(3)).is_copy());
    }

    #[test]
    fn variant_changed() {
        if let Some(enm::Edit::VariantChanged(&None, &Some(3))) = None.diff(&Some(3)).change() {
        } else {
            unreachable!();
        }
    }

    #[test]
    fn associate_change() {
        if let Some(enm::Edit::AssociatedChanged((&1, &2))) = Some(1).diff(&Some(2)).change() {
        } else {
            unreachable!();
        }
    }
}
