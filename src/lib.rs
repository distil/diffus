use std::collections::HashMap;
use std::hash::Hash;

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

// Standard types
impl<'a> Diffable<'a> for String {
    type D = (&'a str, &'a str);

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
        if self == other {
            Edit::Copy
        } else {
            Edit::Change((self.as_ref(), other.as_ref()))
        }
    }
}

// Standard types
impl<'a> Diffable<'a> for i32 {
    type D = (&'a i32, &'a i32);

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
        if self == other {
            Edit::Copy
        } else {
            Edit::Change((self, other))
        }
    }
}


type EditedHashMap<'a, K, V> = HashMap<&'a K, Edit<'a, V>>;

impl<'a, K: Eq + Hash + 'a, V: Diffable<'a> + 'a> Diffable<'a> for HashMap<K, V> {
    type D = EditedHashMap<'a, K, V>;

    fn diff(&'a self, other: &'a Self) -> Edit<Self> {
        let intersection = self.iter()
            .filter_map(|(k, v)| Some((k, (v, other.get(k)?))));

        let unique_self = self.iter()
            .filter(|(k, _)| !other.contains_key(k));

        let unique_other = other.iter()
            .filter(|(k, _)| !self.contains_key(k));

        let value_diffs = unique_other
            .map(|(k, v)| (k, Edit::Insert(v)))
            .chain(
                unique_self
                    .map(|(k, _)| (k, Edit::Remove))
            )
            .chain(
                intersection
                    .map(|(k, (self_v, other_v))| (k, self_v.diff(other_v)))
            )
            .collect::<HashMap<_,_>>();

        if value_diffs.values().any(|v| !v.is_copy()) {
            Edit::Change(value_diffs)
        } else {
            Edit::Copy
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashmap_simple() {
        let unity: HashMap<_,_> = [(1,1), (2,2), (3,3)].iter().cloned().collect();
        let not_unity: HashMap<_,_> = [(1,1), (2,3), (4,4)].iter().cloned().collect();

        if let Edit::Change(diff) = unity.diff(&not_unity) {
            assert!(diff.get(&1).unwrap().is_copy());
            assert_eq!(diff.get(&2).unwrap().change().unwrap(), &(&2, &3));
            assert!(diff.get(&3).unwrap().is_remove());
            assert_eq!(diff.get(&4).unwrap().insert().unwrap(), &4);
        } else {
            unreachable!()
        }
    }


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
