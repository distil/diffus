use diffus::Diffus;

#[derive(Diffus)]
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

    assert_eq!(diff.changed().unwrap().y.changed().unwrap(), ("asdf", "snagins"));
}
