use diffus_derive::{
    Diffus,
};

use diffus::{
    Diffable,
    Edit,
    EditField,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Diffus, Debug)]
    struct Inner {
        x: String,
        y: String,
    }

    #[derive(Diffus, Debug)]
    struct Outer {
        inner: Inner,
        other: i32,
    }

    #[test]
    fn nested() {
        let left = Outer {
            inner: Inner {
                x: "x".to_owned(),
                y: "y left".to_owned(),
            },
            other: 3,
        };
        let right = Outer {
            inner: Inner {
                x: "x".to_owned(),
                y: "y right".to_owned(),
            },
            other: 3,
        };

        let diff = left.diff(&right);

        assert_eq!(
            diff.change().unwrap()
                .inner.change().unwrap()
                .y.change().unwrap(),
            &("y left", "y right")
        );

        if let Edit::Change(diff) = diff {
            match (diff.inner, diff.other) {
                (Edit::Change(diff), Edit::Copy(_)) => {
                    match (diff.x, diff.y) {
                        (EditField::Copy(_), EditField::Change((left, right))) => {
                            assert_eq!(left, "y left");
                            assert_eq!(right, "y right");
                        },
                        _ => unreachable!("here"),
                    }
                },
                _ => unreachable!("there"),
            }
        } else {
            unreachable!("somewhere")
        }

    }
}
