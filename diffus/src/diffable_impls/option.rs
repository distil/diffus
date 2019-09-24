use crate::{edit::{self, enm}, edit::Edit, Diffable};

impl<'a, T: Diffable<'a> + 'a> Diffable<'a> for Option<T> {
    type D = enm::Edit<'a, Self, T::D>;

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
        match (self, other) {
            (None, None) => Edit::Copy,
            (None, _) | (_, None) => Edit::Change(
                enm::Edit::VariantChanged((self, other))
            ),
            (Some(a), Some(b)) => match a.diff(&b) {
                Edit::Copy => Edit::Copy,
                Edit::Change(d) => Edit::Change(enm::Edit::AssociatedChanged(d)),
            },
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
        if let enm::Edit::VariantChanged((&None, &Some(3))) = None.diff(&Some(3)).change().unwrap() {
        } else {
            unreachable!();
        }
    }

    #[test]
    fn associate_change() {
        if let enm::Edit::AssociatedChanged((&1, &2)) = Some(1).diff(&Some(2)).change().unwrap() {
        } else {
            unreachable!();
        }
    }
}
