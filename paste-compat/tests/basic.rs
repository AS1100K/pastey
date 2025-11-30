#![allow(non_snake_case)]

use paste_compat::paste;

#[test]
fn test_case_modifers() {
    paste! {
        let [<Q R S T>] = "QRST";
        let [<LIB:camel env!("CARGO_PKG_NAME"):camel>] = "LibPasteCompact";
        let [<LIB:snake:upper env!("CARGO_PKG_NAME"):camel>] = "L_I_BPasteCompat";
        let [<LIB:snake:lower env!("CARGO_PKG_NAME"):camel>] = "l_i_bPasteCompat";
        let [<LIB:snake env!("CARGO_PKG_NAME"):snake>] = "l_i_bpaste_compat";
        let [<lib_:snake env!("CARGO_PKG_NAME"):snake>] = "lib_paste_compat";
        let [<LIB env!("CARGO_PKG_NAME")>] = "LIBpaste_compat";
    }

    assert_eq!(QRST, "QRST");
    assert_eq!(LibPasteCompat, "LibPasteCompact");
    assert_eq!(L_I_BPasteCompat, "L_I_BPasteCompat");
    assert_eq!(l_i_bPasteCompat, "l_i_bPasteCompat");
    assert_eq!(l_i_bpaste_compat, "l_i_bpaste_compat");
    assert_eq!(lib_paste_compat, "lib_paste_compat");
    assert_eq!(LIBpaste_compat, "LIBpaste_compat");
}

#[test]
fn test_literals() {
    paste! {
        let [<Hello 0>] = "Hello0";
        let [<Hello_:lower 0:camel>] = "hello_0";
        let number_int_f32 = [<10 f32>];
        let number_int_f64 = [<10 f64>];
    }

    assert_eq!(Hello0, "Hello0");
    assert_eq!(hello_0, "hello_0");
    assert_eq!(number_int_f32, 10f32);
    assert_eq!(number_int_f64, 10f64);
}

mod test_to_snake {
    use super::*;

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

mod test_to_snake_edge {
    use super::*;

    macro_rules! m {
        ($id:ident) => {
            paste! {
                const DEFAULT_SNAKE: &str = stringify!([<$id:snake>]);
                const LOWER_SNAKE: &str = stringify!([<$id:snake:lower>]);
                const UPPER_SNAKE: &str = stringify!([<$id:snake:upper>]);
            }
        };
    }

    m!(This__Is_ButATest);

    #[test]
    fn test_to_snake() {
        assert_eq!(DEFAULT_SNAKE, "this__is_but_a_test");
        assert_eq!(LOWER_SNAKE, "this__is_but_a_test");
        assert_eq!(UPPER_SNAKE, "THIS__IS_BUT_A_TEST");
    }
}

mod test_to_camel {
    use super::*;

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

mod test_to_camel_edge {
    use super::*;

    macro_rules! m {
        ($id:ident) => {
            paste! {
                const DEFAULT_CAMEL: &str = stringify!([<$id:camel>]);
                const LOWER_CAMEL: &str = stringify!([<$id:camel:lower>]);
                const UPPER_CAMEL: &str = stringify!([<$id:camel:upper>]);
            }
        };
    }

    m!(___this_is__but_a____test);

    #[test]
    fn test_to_camel() {
        assert_eq!(DEFAULT_CAMEL, "ThisIsButATest");
        assert_eq!(LOWER_CAMEL, "thisisbutatest");
        assert_eq!(UPPER_CAMEL, "THISISBUTATEST");
    }
}
