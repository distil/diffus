use diffus::{
    Diffable,
};
use diffus_derive::{
    Diffus,
};

#[derive(Diffus, Debug)]
struct Inner {
    x: String,
    y: String,
}

#[derive(Diffus, Debug)]
struct Outer {
    inner: Inner,
}

fn main() {
    let left = Outer {
        inner: Inner {
            x: "x".to_owned(),
            y: "y left".to_owned(),
        }
    };
    let right = Outer {
        inner: Inner {
            x: "x".to_owned(),
            y: "y right".to_owned(),
        }
    };

    let diff = left.diff(&right);

    assert_eq!(
        diff.change().unwrap()
            .inner.change().unwrap()
            .y.change().unwrap(),
        &("y left", "y right")
    );

    println!("left: {:?}", left);
    println!("right: {:?}", right);
    println!("done");
}
