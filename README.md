# Diffus
Finds the difference between two instances of any data structure.

## Diffus in action
```rust
use diffus::{edit, Diffable, Diffus};

#[derive(Diffus)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let left_point = Point { x: 1, y: 2 };
    let right_point = Point { x: 1, y: 3 };

    let diff = left_point.diff(&right_point);

    match diff {
        edit::Edit::Copy => println!("point: no difference"),
        edit::Edit::Change(EditedPoint { x, y }) => {
            match x {
                edit::Edit::Copy => println!("x: no difference"),
                edit::Edit::Change((left_x, right_x)) => println!("x: {} => {}", left_x, right_x),
            }
            match y {
                edit::Edit::Copy => println!("y: no difference"),
                edit::Edit::Change((left_y, right_y)) => println!("y: {} => {}", left_y, right_y),
            }
        }
    }
}
```

### Diffus on maps
Difference between maps is done through its keys. The identity of an element comes from its associated key, and it otherwise works in the same way as for collections with `Same`.

The possible edits of a map are `Copy`, `Insert`, `Remove`, `Change`
```rust
use diffus::{edit, Diffable};

fn main() {
    let unity: std::collections::HashMap<_, _> =
        [(1, 1), (2, 2), (3, 3)].iter().cloned().collect();
    let not_unity: std::collections::HashMap<_, _> =
        [(1, 1), (2, 3), (4, 4)].iter().cloned().collect();

    if let edit::Edit::Change(diff) = unity.diff(&not_unity) {
        assert!(diff[&1].is_copy());
        assert_eq!(diff[&2].change().unwrap(), &(&2, &3));
        assert!(diff[&3].is_remove());
        assert_eq!(diff[&4].insert().unwrap(), &4);
    } else {
        unreachable!()
    }
}
```

### Diffus on collections
Difference between collections is done through the Longest Common Subsequence (LCS) algorithm and with additional support for objects that have changed values but kept its `Same` "identity".

Strings are considered as collections.

The possible edits of a collection are `Copy`, `Insert`, `Remove`, `Change`.

```rust
use diffus::{edit::{self, collection}, Same, Diffable, Diffus};

#[derive(Diffus, Debug)]
struct Identified {
    id: u32,
    value: u32,
}

impl Same for Identified {
    fn same(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn main() {
    let left = vec![
        Identified { id: 1, value: 0 },
        Identified { id: 2, value: 0 },
        Identified { id: 3, value: 0 },
        Identified { id: 4, value: 0 },
        Identified { id: 5, value: 0 },
        Identified { id: 6, value: 0 },
        Identified { id: 7, value: 0 },
    ];
    let right = vec![
        Identified { id: 1, value: 0 },
        Identified { id: 2, value: 1 },
        Identified { id: 4, value: 0 },
        Identified { id: 3, value: 0 },
        Identified { id: 5, value: 0 },
        Identified { id: 6, value: 0 },
    ];

    let diff = left.diff(&right);

    match diff {
        edit::Edit::Copy => println!("no difference"),
        edit::Edit::Change(diff) => {
            diff.into_iter().map(|edit| {
                match edit {
                    collection::Edit::Copy(elem) => println!("copy: {:?}", elem),
                    collection::Edit::Insert(elem) => println!("insert: {:?}", elem),
                    collection::Edit::Remove(elem) => println!("remove: {:?}", elem),
                    collection::Edit::Change(EditedIdentified { id, value}) => {
                        println!("changed:");
                        match id {
                            edit::Edit::Copy => println!("    copy: id"),
                            edit::Edit::Change((left_id, right_id)) => println!("    id: {} => {}", left_id, right_id),
                        }
                        match value {
                            edit::Edit::Copy => println!("    copy: value"),
                            edit::Edit::Change((left_value, right_value)) => println!("    value: {} => {}", left_value, right_value),
                        }
                    }
                };
            }).collect::<Vec<_>>();
        },
    };
}
```


### Diffus on enums
Difference between two enums works as expected, it separates variant changes from associated variant field changes.

Possible edits of an enum are `Copy`, `VariantChanged`, `AssociatedChanged`.

```rust
use diffus::{edit, Diffable, Diffus};

#[derive(Diffus, Debug, PartialEq)]
enum Test {
    A,
    B(String),
    Bd(String, u32),
    C { x: u32 },
    Cd { x: u32, y: String },
}

fn main() {
    let left = Test::Cd {
        x: 42,
        y: "Bilbo Baggins".to_owned(),
    };
    let right = Test::Cd {
        x: 42,
        y: "Frodo Baggins".to_owned(),
    };
    if let edit::Edit::Change(edit::enm::Edit::AssociatedChanged(
        EditedTest::Cd { x, y },
    )) = left.diff(&right)
    {
        assert!(x.is_copy());
        assert!(y.is_change());
    } else {
        unreachable!()
    }

    let left = Test::Cd {
        x: 42,
        y: "Bilbo Baggins".to_owned(),
    };
    let right = Test::B("Frodo Baggins".to_owned());
    if let edit::Edit::Change(edit::enm::Edit::VariantChanged(l, r)) =
        left.diff(&right)
    {
        assert_eq!(&left, l);
        assert_eq!(&right, r);
    } else {
        unreachable!()
    }
}
```


### Custom difference with diffus
Differences can easily be specialized to suit your needs.

```rust
use diffus::{edit, Diffable};

struct Secret(String);

impl<'a> Diffable<'a> for Secret {
    type Diff = ();

    fn diff(&'a self, other: &'a Self) -> edit::Edit<Self::Diff> {
        if self.0 == other.0 {
            edit::Edit::Copy
        } else {
            edit::Edit::Change(())
        }
    }
}

fn main() {
    assert_eq!(
        Secret("Something".to_owned()).diff(&Secret("Else".to_owned()))
            .change().unwrap(),
        &()
    );
}
```
