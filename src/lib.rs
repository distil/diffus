use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
struct Diff<'a, K: Eq + Hash, V> {
    unique_left: HashMap<&'a K, &'a V>,
    intersection: HashMap<&'a K, &'a V>,
    unique_right: HashMap<&'a K, &'a V>,

    value_equal: HashMap<&'a K, &'a V>,
    value_diff: HashMap<&'a K, &'a V>,
}

impl<'a, K: Eq + Hash, V: PartialEq> Diff<'a, K, V> {
    fn new(
        left: &'a HashMap<K, V>,
        right: &'a HashMap<K, V>,
    ) -> Self {
        let intersection = left.iter()
            .filter(|(k, _)| right.contains_key(k))
            .collect::<HashMap<_, _>>();

        let unique_left = left.iter()
            .filter(|(k, _)| !right.contains_key(k))
            .collect::<HashMap<_, _>>();

        let unique_right = right.iter()
            .filter(|(k, _)| !left.contains_key(k))
            .collect::<HashMap<_, _>>();

        let (value_equal, value_diff): (HashMap<_, _>, HashMap<_, _>) = intersection.iter()
            .partition(|(k, v)| {
                right.get(k) == Some(**v)
            });

        Diff {
            unique_left,
            intersection,
            unique_right,
            value_equal,
            value_diff,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let a = [(1, 1), (2, 2)].iter().cloned().collect();
        let b = [(1, 0), (2, 2)].iter().cloned().collect();

        dbg!(Diff::new(&a, &b));
    }
}
