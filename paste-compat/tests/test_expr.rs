#![allow(clippy::let_underscore_untyped)]
#![allow(non_camel_case_types)]

use paste_compat::paste;

#[test]
fn test_literal_suffix() {
    macro_rules! literal {
        ($bit:tt) => {
            paste!([<1_u $bit>])
        };
    }

    assert_eq!(literal!(32), 1);
}

#[test]
fn test_underscore() {
    paste! {
        const A_B: usize = 0;
        assert_eq!([<A _ B>], 0);
    }
}

#[test]
fn test_lifetime() {
    paste! {
        #[allow(dead_code)]
        struct S<[<'d e>]> {
            q: &[<'d e>] str,
        }
    }
}

#[test]
fn test_keyword() {
    paste! {
        struct [<F move>];

        let _ = Fmove;
    }
}

#[test]
fn test_literal_str() {
    paste! {
        #[allow(non_camel_case_types)]
        struct [<Foo "Bar-Baz">];

        let _ = FooBar_Baz;
    }
}

#[test]
fn test_raw_identifier() {
    paste! {
        struct [<F r#move>];

        let _ = Fmove;
    }
}

#[test]
fn test_false_start() {
    trait Trait {
        fn f() -> usize;
    }

    struct S;

    impl Trait for S {
        fn f() -> usize {
            0
        }
    }

    paste! {
        let x = [<S as Trait>::f()];
        assert_eq!(x[0], 0);
    }
}

#[test]
fn test_empty() {
    paste! {
        assert_eq!(stringify!([<y y>]), "yy");
        assert_eq!(stringify!([<>]).replace(' ', ""), "[<>]");
    }
}

#[rustversion::since(1.46)]
mod test_local_setter {
    // https://github.com/dtolnay/paste/issues/7

    use paste_compat::paste;

    #[derive(Default)]
    struct Test {
        val: i32,
    }

    impl Test {
        fn set_val(&mut self, arg: i32) {
            self.val = arg;
        }
    }

    macro_rules! setter {
        ($obj:expr, $field:ident, $value:expr) => {
            paste! { $obj.[<set_ $field>]($value); }
        };

        ($field:ident, $value:expr) => {{
            let mut new = Test::default();
            setter!(new, val, $value);
            new
        }};
    }

    #[test]
    fn test_local_setter() {
        let a = setter!(val, 42);
        assert_eq!(a.val, 42);
    }
}

// https://github.com/dtolnay/paste/issues/85
#[test]
fn test_top_level_none_delimiter() {
    macro_rules! clone {
        ($val:expr) => {
            paste! {
                $val.clone()
            }
        };
    }

    #[derive(Clone)]
    struct A;

    impl A {
        fn consume_self(self) {
            let _ = self;
        }
    }

    clone!(&A).consume_self();
}
