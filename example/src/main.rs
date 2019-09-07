use diffus::{
    Diffable,
};
use diffus_derive::{
    Diffus,
};

#[derive(Diffus, Debug)]
struct Foo {
    x: String,
    y: String,
}

fn main() {
    let left = Foo {
        x: "bilbo".to_owned(),
        y: "asdf".to_owned(),
    };
    let right = Foo {
        x: "bilbo".to_owned(),
        y: "snagins".to_owned(),
    };

    let diff = left.diff(&right);

    assert_eq!(
        diff.change().unwrap().y.change().unwrap(),
        &("asdf", "snagins")
    );

    println!("left: {:?}", left);
    println!("right: {:?}", right);
    println!("done");
}
