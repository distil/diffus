#[cfg(test)]
#[allow(unused_variables)]
#[allow(dead_code)]
mod test {
    use diffus_derive::Diffus;

    use diffus::{self, edit, Diffable};

    mod hide {
        use super::*;

        #[derive(Diffus)]
        pub struct Inside {
            pub p: u32,
        }

        impl Inside {
            pub fn new(p: u32) -> Self {
                Inside { p }
            }
        }
    }

    #[test]
    fn vis_check() {
        if let edit::Edit::Change(hide::EditedInside {
            p: edit::Edit::Change(diff),
            ..
        }) = hide::Inside::new(0).diff(&hide::Inside::new(1))
        {
            assert_eq!(diff, (&0, &1));
        } else {
            unreachable!()
        }
    }

    #[derive(Diffus)]
    struct Lifetime<'a>(&'a u32);

    #[derive(Diffus, Debug, PartialEq)]
    struct Identified {
        id: u32,
        value: u32,
    }

    impl diffus::Same for Identified {
        fn same(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    #[test]
    fn non_trivial_same_collection() {
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

        use diffus::edit::{self, collection};

        if let edit::Edit::Change(diff) = diff {
            let diff = diff.into_iter().collect::<Vec<_>>();

            if let (
                &collection::Edit::Change(EditedIdentified {
                    id: edit::Edit::Copy(&2),
                    value: edit::Edit::Change((&0, &1)),
                }),
                &collection::Edit::Remove(&Identified { id: 3, value: 0 }),
                &collection::Edit::Copy(&Identified { id: 4, value: 0 }),
                &collection::Edit::Insert(&Identified { id: 3, value: 0 }),
            ) = (&diff[1], &diff[2], &diff[3], &diff[4])
            {
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    #[derive(Diffus)]
    enum NestedTest {
        T { test: Test },
    }

    #[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
    #[derive(Debug, Diffus, PartialEq, Eq)]
    enum Test {
        A,
        B(String),
        Bd(String, u32),
        C { x: u32 },
        Cd { x: u32, y: String },
    }

    /*
     * Verify enum refering to own type via hashmap
     */
    #[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
    #[derive(Debug, Diffus, PartialEq)]
    enum RecursiveHashMap {
        Node(std::collections::HashMap<u32, RecursiveHashMap>),
        Empty,
    }

    /*
     * Verify enum refering to own type via box
     */
    #[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
    #[derive(Debug, Diffus, PartialEq)]
    enum RecursiveBox {
        Boxed(Box<RecursiveBox>),
    }

    /*
     * Verify enums with only Unit variants.
     */
    #[derive(Diffus)]
    enum EnumNoLifetimeParameter {
        A,
        B,
    }

    mod visibility_test {
        /*
         * Verify that the visibility of the Edited version is inherited.
         */
        use diffus_derive::Diffus;

        #[derive(Diffus)]
        pub struct VisTestStructUnit;

        #[derive(Diffus)]
        pub struct VisTestStructTuple(u32);

        #[derive(Diffus)]
        pub struct VisTestStruct {
            x: u32,
        }

        #[derive(Diffus)]
        pub enum VisTestEnum {
            A,
            B(u32),
            C { x: u32 },
        }
    }

    #[test]
    fn enm_nested_test() {
        let left = NestedTest::T {
            test: Test::C { x: 32 },
        };
        let right = NestedTest::T {
            test: Test::C { x: 43 },
        };

        let diff = left.diff(&right);

        if let diffus::edit::enm::Edit::AssociatedChanged(EditedNestedTest::T { test }) =
            diff.change().unwrap()
        {
            if let diffus::edit::enm::Edit::AssociatedChanged(EditedTest::C { x }) =
                test.change().unwrap()
            {
                assert_eq!(x.change(), Some(&(&32, &43)));
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
    }

    #[test]
    fn enm_associated_not_change_tuple_variant() {
        let left = Test::Bd("Bilbo Baggins".to_owned(), 42);
        let right = Test::Bd("Bilbo Baggins".to_owned(), 42);

        assert!(left.diff(&right).is_copy());
    }

    #[test]
    fn enm_associated_not_change() {
        let left = Test::Cd {
            x: 42,
            y: "Bilbo Baggins".to_owned(),
        };
        let right = Test::Cd {
            x: 42,
            y: "Bilbo Baggins".to_owned(),
        };

        assert!(left.diff(&right).is_copy());
    }

    #[test]
    fn enm_associated_change() {
        let left = Test::Cd {
            x: 42,
            y: "Bilbo Baggins".to_owned(),
        };
        let right = Test::Cd {
            x: 42,
            y: "Frodo Baggins".to_owned(),
        };
        if let diffus::edit::Edit::Change(diffus::edit::enm::Edit::AssociatedChanged(
            EditedTest::Cd { x, y },
        )) = left.diff(&right)
        {
            assert!(x.is_copy());
            assert!(y.is_change());
        } else {
            unreachable!()
        }
    }

    #[test]
    fn enm_variant_change() {
        let left = Test::Cd {
            x: 42,
            y: "Bilbo Baggins".to_owned(),
        };
        let right = Test::B("Frodo Baggins".to_owned());
        if let diffus::edit::Edit::Change(diffus::edit::enm::Edit::VariantChanged(l, r)) =
            left.diff(&right)
        {
            assert_eq!(&left, l);
            assert_eq!(&right, r);
        } else {
            unreachable!()
        }
    }

    #[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
    #[derive(Diffus, Debug, PartialEq)]
    struct Inner {
        x: String,
        y: u32,
    }

    #[derive(Diffus, Debug, PartialEq)]
    struct Unit;

    #[derive(Diffus, Debug, PartialEq)]
    struct Unnamed(u32, String);

    #[derive(Diffus, Debug, PartialEq)]
    struct Outer {
        inner: Inner,
        lit: i32,
    }

    #[test]
    fn nested() {
        let left = Outer {
            inner: Inner {
                x: "x".to_owned(),
                y: 13,
            },
            lit: 3,
        };
        let right = Outer {
            inner: Inner {
                x: "x".to_owned(),
                y: 37,
            },
            lit: 3,
        };

        let diff = left.diff(&right);

        assert_eq!(
            diff.change()
                .unwrap()
                .inner
                .change()
                .unwrap()
                .y
                .change()
                .unwrap(),
            &(&13, &37)
        );
    }

    #[cfg(feature = "serialize-impl")]
    mod serialize {
        use super::*;

        #[derive(Diffus, Default, serde::Serialize)]
        struct SB {
            u: u32,
        }

        #[derive(Diffus, Default, serde::Serialize)]
        struct SA {
            b: SB,
            s: String,
        }

        #[test]
        fn example() {
            use serde_json::*;

            let left = &SA {
                b: SB { u: 34 },
                s: "string".to_string(),
            };

            let string = to_string(&left.diff(&SA {
                b: SB { u: 34 },
                s: "strga".to_string(),
            }))
            .unwrap();

            let json: Value = from_str(&string).unwrap();

            assert_eq!(
                json,
                serde_json::json!({
                    "Change": {
                        "b": {
                            "Copy": {
                                "u": 34,
                            }
                        },
                        "s": {
                            "Change": [
                                {
                                    "Copy": "s",
                                },
                                {
                                    "Copy": "t",
                                },
                                {
                                    "Copy": "r",
                                },
                                {
                                    "Remove": "i",
                                },
                                {
                                    "Remove": "n",
                                },
                                {
                                    "Copy": "g",
                                },
                                {
                                    "Insert": "a",
                                },
                            ]
                        }
                    }
                })
            );
        }
    }

    #[test]
    fn struct_containing_str() {
        #[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
        #[derive(Diffus, Debug, PartialEq)]
        struct A<'a> {
            a: &'a str,
        }

        let a = A { a: "a" };
        let ap = A { a: "a'" };

        let diff = a.diff(&ap);
        let actual = diff.change().unwrap().a.change().unwrap();

        use diffus::edit::string;

        assert_eq!(
            actual,
            &vec![string::Edit::Copy('a'), string::Edit::Insert('\''),]
        );
    }
}
