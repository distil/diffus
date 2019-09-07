mod diffable_impls;
mod edit;

pub use edit::Edit;

pub trait Diffable<'a>: Sized {
    type D: Sized + 'a;

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self>;
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
