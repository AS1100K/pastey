#![allow(clippy::let_underscore_untyped)]

mod test_basic {
    use pastey::paste;

    struct Struct;

    paste! {
        impl Struct {
            fn [<a b c>]() {}
        }
    }

    #[test]
    fn test() {
        Struct::abc();
    }
}

mod test_in_impl {
    use pastey::paste;

    struct Struct;

    impl Struct {
        paste! {
            fn [<a b c>]() {}
        }
    }

    #[test]
    fn test() {
        Struct::abc();
    }
}

mod test_none_delimited_single_ident {
    use pastey::paste;

    macro_rules! m {
        ($id:ident) => {
            paste! {
                fn f() -> &'static str {
                    stringify!($id)
                }
            }
        };
    }

    m!(i32x4);

    #[test]
    fn test() {
        assert_eq!(f(), "i32x4");
    }
}

mod test_none_delimited_single_lifetime {
    use pastey::paste;

    macro_rules! m {
        ($life:lifetime) => {
            paste! {
                pub struct S<$life>(#[allow(dead_code)] pub &$life ());
                impl<$life> S<$life> {
                    fn f() {}
                }
            }
        };
    }

    m!('a);

    #[test]
    fn test() {
        S::f();
    }
}

mod test_to_lower {
    use pastey::paste;

    macro_rules! m {
        ($id:ident) => {
            paste! {
                fn [<my_ $id:lower _here>](_arg: u8) -> &'static str {
                    stringify!([<$id:lower>])
                }
            }
        };
    }

    m!(Test);

    #[test]
    fn test_to_lower() {
        assert_eq!(my_test_here(0), "test");
    }
}

mod test_to_upper {
    use pastey::paste;

    macro_rules! m {
        ($id:ident) => {
            paste! {
                const [<MY_ $id:upper _HERE>]: &str = stringify!([<$id:upper>]);
            }
        };
    }

    m!(Test);

    #[test]
    fn test_to_upper() {
        assert_eq!(MY_TEST_HERE, "TEST");
    }
}

mod test_to_snake {
    use pastey::paste;

    macro_rules! m {
        ($id:ident) => {
            paste! {
                const DEFAULT_SNAKE: &str = stringify!([<$id:snake>]);
                const LOWER_SNAKE: &str = stringify!([<$id:snake:lower>]);
                const UPPER_SNAKE: &str = stringify!([<$id:snake:upper>]);
            }
        };
    }

    m!(ThisIsButATest);

    #[test]
    fn test_to_snake() {
        assert_eq!(DEFAULT_SNAKE, "this_is_but_a_test");
        assert_eq!(LOWER_SNAKE, "this_is_but_a_test");
        assert_eq!(UPPER_SNAKE, "THIS_IS_BUT_A_TEST");
    }
}

mod test_to_camel {
    use pastey::paste;

    macro_rules! m {
        ($id:ident) => {
            paste! {
                const DEFAULT_CAMEL: &str = stringify!([<$id:camel>]);
                const LOWER_CAMEL: &str = stringify!([<$id:camel:lower>]);
                const UPPER_CAMEL: &str = stringify!([<$id:camel:upper>]);
            }
        };
    }

    m!(this_is_but_a_test);

    #[test]
    fn test_to_camel() {
        assert_eq!(DEFAULT_CAMEL, "ThisIsButATest");
        assert_eq!(LOWER_CAMEL, "thisisbutatest");
        assert_eq!(UPPER_CAMEL, "THISISBUTATEST");
    }
}

mod test_to_lower_camel {
    use pastey::paste;

    macro_rules! m {
        ($id:ident) => {
            paste! {
                const DEFAULT_LOWER_CAMEL: &str = stringify!([<$id:lower_camel>]);
            }
        };
    }

    m!(this_is_but_a_test);

    #[test]
    fn test_to_lower_camel() {
        assert_eq!(DEFAULT_LOWER_CAMEL, "thisIsButATest");
    }
}

mod test_to_camel_edge {
    use pastey::paste;

    macro_rules! m {
        ($id:ident) => {
            paste! {
                const DEFAULT_CAMEL_EDGE: &str = stringify!([<$id:camel_edge>]);
                const LOWER_CAMEL_EDGE: &str = stringify!([<$id:camel_edge:lower>]);
                const UPPER_CAMEL_EDGE: &str = stringify!([<$id:camel_edge:upper>]);
            }
        };
    }

    m!(__this_is_but_a__test);

    #[test]
    fn test_to_camel_edge() {
        assert_eq!(DEFAULT_CAMEL_EDGE, "__ThisIsButA_Test");
        assert_eq!(LOWER_CAMEL_EDGE, "__thisisbuta_test");
        assert_eq!(UPPER_CAMEL_EDGE, "__THISISBUTA_TEST");
    }
}

mod test_doc_expr {
    // https://github.com/dtolnay/paste/issues/29

    use pastey::paste;

    macro_rules! doc_expr {
        ($doc:expr) => {
            paste! {
                #[doc = $doc]
                pub struct S;
            }
        };
    }

    doc_expr!(stringify!(...));

    #[test]
    fn test_doc_expr() {
        let _: S;
    }
}

mod test_type_in_path {
    // https://github.com/dtolnay/paste/issues/31

    use pastey::paste;

    mod keys {
        #[derive(Default)]
        pub struct Mib<T = ()>(std::marker::PhantomData<T>);
    }

    macro_rules! types {
        ($mib:ty) => {
            paste! {
                #[derive(Default)]
                pub struct S(pub keys::$mib);
            }
        };
    }

    macro_rules! write {
        ($fn:ident, $field:ty) => {
            paste! {
                pub fn $fn() -> $field {
                    $field::default()
                }
            }
        };
    }

    types! {Mib<[usize; 2]>}
    write! {get_a, keys::Mib}
    write! {get_b, usize}

    #[test]
    fn test_type_in_path() {
        let _: S;
        let _ = get_a;
        let _ = get_b;
    }
}

mod test_type_in_fn_arg {
    // https://github.com/dtolnay/paste/issues/38

    use pastey::paste;

    fn jit_address(_node: ()) {}

    macro_rules! jit_reexport {
        ($fn:ident, $arg:ident : $typ:ty) => {
            paste! {
                pub fn $fn($arg: $typ) {
                    [<jit_ $fn>]($arg);
                }
            }
        };
    }

    jit_reexport!(address, node: ());

    #[test]
    fn test_type_in_fn_arg() {
        let _ = address;
    }
}

mod test_pat_in_expr_position {
    // https://github.com/xiph/rav1e/pull/2324/files

    use pastey::paste;

    macro_rules! rav1e_bad {
        ($e:pat) => {
            paste! {
                #[test]
                fn test() {
                    let _ = $e;
                }
            }
        };
    }

    rav1e_bad!(std::fmt::Error);
}

mod test_raw_mode {
    use pastey::paste;

    macro_rules! m{
        ($name:ident $(- $name_tail:ident)*) => {
            paste!{
                struct [< $name:camel $( $name_tail)* >];

                impl [< $name:camel $( $name_tail)* >] {
                    fn [< # $name:snake $( _ $name_tail:snake)* >]() {}
                }

            }
        }
    }

    m!(loop);
    m!(loop - xyz);
    m!(dyn);
    m!(unsafe);

    #[test]
    fn test_fn() {
        let _ = Loop::r#loop();
        let _ = Loopxyz::loop_xyz();
        let _ = Dyn::r#dyn();
        let _ = Unsafe::r#unsafe();
    }
}

mod test_join {
    use pastey::paste;

    macro_rules! m {
        ($name:ident, $func:ident) => {
            paste! {
                struct [< # $name:camel >];

                impl [< # $name:camel >] {
                    fn [< $func:snake >](&self) {}

                    fn test()  {
                        [< # $name:camel >].[< $func:snake >]()
                    }
                }
            }
        };
    }

    m!(loop, welcome);

    #[test]
    fn test_fn() {
        Loop::test();
    }
}

mod test_replace {
    use pastey::paste;

    macro_rules! m {
        ($($command:ident),+) => {
            paste! {
                $(
                    pub struct $command {}
                )*

                pub enum Command {
                    $(
                        [< $command:replace("Command", "") >] ( $command )
                    ),*
                }
            }
        };
    }

    m! {
        CommandFoo,
        CommandBar
    }

    #[test]
    fn test_replace() {
        let _ = Command::Bar(CommandBar {});
        let _ = Command::Foo(CommandFoo {});
    }
}
