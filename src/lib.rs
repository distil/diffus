mod diffable_impls;

pub trait Diffable<'a>: Sized {
    type D: Sized + 'a;

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self>;
}

pub enum Edit<'a, T: Diffable<'a>> {
    Insert(&'a T),
    Remove,
    Copy,
    Change(T::D),
}

impl<'a, T: Diffable<'a>> Edit<'a, T> {
    pub fn is_insert(&self) -> bool {
        if let Self::Insert(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_remove(&self) -> bool {
        if let Self::Remove = self {
            true
        } else {
            false
        }
    }
    pub fn is_copy(&self) -> bool {
        if let Self::Copy = self {
            true
        } else {
            false
        }
    }
    pub fn is_change(&self) -> bool {
        if let Self::Change(_) = self {
            true
        } else {
            false
        }
    }
    pub fn insert(&self) -> Option<&'a T> {
        if let Self::Insert(value) = self {
            Some(value)
        } else {
            None
        }
    }
    pub fn change(&self) -> Option<&T::D> {
        if let Self::Change(value_diff) = self {
            Some(value_diff)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn simple() {
        let foo = Foo {
            x: "Frodo".to_owned(),
            y: "Baggins".to_owned(),
        };

        let bar = Foo {
            x: "Bilbo".to_owned(),
            y: "Baggins".to_owned(),
        };

        let diff = foo.diff(&bar);

        if let Edit::Change(diff) = diff {
            match (diff.x, diff.y) {
                (Edit::Change((left, right)), Edit::Copy) => {
                    assert_eq!(left, "Frodo");
                    assert_eq!(right, "Bilbo");
                },
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }

    #[test]
    fn nested() {
        let foo = Bar {
            x: Foo {
                x: "Frodo".to_owned(),
                y: "Baggins".to_owned(),
            },
            y: "Shire".to_owned(),
        };

        let bar = Bar {
            x: Foo {
                x: "Bilbo".to_owned(),
                y: "Baggins".to_owned(),
            },
            y: "Shire".to_owned(),
        };

        let diff = foo.diff(&bar);

        if let Edit::Change(diff) = diff {
            match (diff.x, diff.y) {
                (Edit::Change(diff), Edit::Copy) => {
                    match (diff.x, diff.y) {
                        (Edit::Change((left, right)), Edit::Copy) => {
                            assert_eq!(left, "Frodo");
                            assert_eq!(right, "Bilbo");
                        },
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }
}
