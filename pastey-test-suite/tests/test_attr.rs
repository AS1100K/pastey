#![allow(clippy::let_underscore_untyped)]

use pastey::paste;
use pastey_test_suite::paste_test;

paste! {
    mod test_inner_attr_mod {
        #![allow(dead_code)]
        pub struct InnerAttrItem;
    }
}

#[test]
fn test_inner_attr_poundbang() {
    let _: test_inner_attr_mod::InnerAttrItem;
}

#[test]
fn test_attr() {
    paste! {
        #[paste_test(k = "val" "ue")]
        struct A;

        #[paste_test(k = "val" "ue")]
        struct B;

        #[paste_test(k = "val" "ue")]
        struct C;

        #[paste_test(k = "va" [<l u>] e)]
        struct D;
    }

    let _ = A;
    let _ = B;
    let _ = C;
    let _ = D;
}

#[test]
fn test_paste_cfg() {
    macro_rules! m {
        ($ret:ident, $width:expr) => {
            paste! {
                #[cfg(any(feature = "protocol_feature_" $ret:snake, target_pointer_width = "" $width))]
                fn new() -> $ret { todo!() }
            }
        };
    }

    struct Paste;

    #[cfg(target_pointer_width = "64")]
    m!(Paste, 64);
    #[cfg(target_pointer_width = "32")]
    m!(Paste, 32);

    let _ = new;
}

#[test]
fn test_path_in_attr() {
    macro_rules! m {
        (#[x = $x:ty]) => {
            stringify!($x)
        };
    }

    let ty = paste! {
        m!(#[x = foo::Bar])
    };

    assert_eq!("foo::Bar", ty);
}

#[test]
fn test_paste_raw_mode() {
    macro_rules! m {
        ($name:ident) => {
            paste! {
                #[paste_test(k = "value" [< # $name >])]
                struct [<# $name:camel>];
            }
        };
    }

    m!(loop);

    let _ = Loop;
}

#[test]
fn test_leading_colons_attr() {
    paste! {
        #[::core::prelude::v1::derive(Copy, Clone)]
        struct LeadingColons(u32);
    }

    let a = LeadingColons(1);
    let b = a;
    let _ = b;
}

#[test]
fn test_doc_raw_mode_unreachable_note() {
    paste! {
        #[doc = "Hello " "World"]
        pub struct DocStringPaste;
    }
    let _ = DocStringPaste;
}

#[test]
fn test_attr_paren_comma_split() {
    paste! {
        #[derive(Clone, Copy)]
        struct CommaInParen(u8);
    }
    let a = CommaInParen(1);
    let b = a;
    let _ = b;
}

#[test]
fn test_attr_paren_no_comma_final_expand() {
    paste! {
        #[cfg(not(any()))]
        fn _never_compiled() {}
    }
}

#[test]
fn test_single_colon_before_ident_fallthrough() {
    paste! {
        #[cfg_attr(not(all()), allow(: foo, bar))]
        struct SingleColonBeforeIdentFallthrough;
    }
    let _ = SingleColonBeforeIdentFallthrough;
}

#[test]
fn test_consecutive_idents_in_paren_fallthrough() {
    paste! {
        #[cfg_attr(not(all()), allow(a, b c, d))]
        struct ConsecutiveIdentsFallthrough;
    }
    let _ = ConsecutiveIdentsFallthrough;
}

#[test]
fn test_triple_colon_in_paren_fallthrough() {
    paste! {
        #[cfg_attr(not(all()), allow(foo:::bar))]
        struct TripleColonFallthrough;
    }
    let _ = TripleColonFallthrough;
}

#[test]
fn test_eq_before_ident_fallthrough() {
    paste! {
        #[cfg_attr(not(all()), allow(= foo, bar))]
        struct EqBeforeIdentFallthrough;
    }
    let _ = EqBeforeIdentFallthrough;
}

#[test]
fn test_single_token_doc_no_paste() {
    paste! {
        #[doc = "hello"]
        struct SingleTokenDoc;
    }
    let _ = SingleTokenDoc;
}

#[test]
fn test_literal_after_ident_not_group() {
    paste! {
        #[cfg_attr(not(all()), allow(foo 42, bar))]
        struct LiteralAfterIdentNotGroup;
    }
    let _ = LiteralAfterIdentNotGroup;
}

#[test]
fn test_paren_after_double_colon_path() {
    paste! {
        #[cfg_attr(not(all()), allow(foo :: (bar), baz))]
        struct ParenAfterDoubleColonPath;
    }
    let _ = ParenAfterDoubleColonPath;
}

#[test]
fn test_brace_group_after_ident_not_paren() {
    paste! {
        #[cfg_attr(not(all()), allow(foo{bar}, baz))]
        struct BraceGroupAfterIdent;
    }
    let _ = BraceGroupAfterIdent;
}

#[test]
fn test_byte_literal_not_stringlike() {
    paste! {
        #[cfg_attr(not(all()), allow(ident = b"hello"))]
        struct ByteLiteralNotStringlike;
    }
    let _ = ByteLiteralNotStringlike;
}

#[test]
fn test_apostrophe_punct_stringlike() {
    paste! {
        #[cfg_attr(not(all()), allow(ident = 'lifetime "hello"))]
        struct ApostrophePunctStringlike;
    }
    let _ = ApostrophePunctStringlike;
}

#[test]
fn test_joint_colon_not_stringlike() {
    paste! {
        #[cfg_attr(not(all()), allow(ident = :: "hello"))]
        struct JointColonNotStringlike;
    }
    let _ = JointColonNotStringlike;
}

#[test]
fn test_non_colon_punct_not_stringlike() {
    paste! {
        #[cfg_attr(not(all()), allow(ident = + "hello"))]
        struct NonColonPunctNotStringlike;
    }
    let _ = NonColonPunctNotStringlike;
}

#[test]
fn test_non_paste_bracket_in_attr_content() {
    paste! {
        #[cfg_attr(not(all()), allow([deprecated]))]
        struct NonPasteBracketInAttrContent;
    }
    let _ = NonPasteBracketInAttrContent;
}

#[test]
fn test_lifetime_paste_bracket_in_attr() {
    paste! {
        #[cfg_attr(not(all()), allow([<'a>]))]
        struct LifetimePasteBracketInAttr;
    }
    let _ = LifetimePasteBracketInAttr;
}

macro_rules! raw_ident_paste_in_attr {
    ($kw:ident) => {
        paste! {
            #[cfg_attr(not(all()), allow([<# $kw>]))]
            struct RawIdentPasteInAttr;
        }
    };
}
raw_ident_paste_in_attr!(loop);

#[test]
fn test_raw_ident_paste_bracket_in_attr() {
    let _ = RawIdentPasteInAttr;
}

#[test]
fn test_char_unicode_paste_bracket_in_attr() {
    paste! {
        #[cfg_attr(not(all()), allow([<'\u{61}'>]))]
        struct CharUnicodePasteBracketInAttr;
    }
    let _ = CharUnicodePasteBracketInAttr;
}

#[test]
fn test_raw_str_paste_bracket_in_attr() {
    paste! {
        #[cfg_attr(not(all()), allow([<r"hello">]))]
        struct RawStrPasteBracketInAttr;
    }
    let _ = RawStrPasteBracketInAttr;
}

#[test]
fn test_doc_attr_r_hash_prefix_stripped() {
    macro_rules! get_doc {
        (#[doc = $lit:tt]) => {
            $lit
        };
    }
    let doc = paste! {
        get_doc!(#[doc = "r#" "hello"])
    };
    assert_eq!(doc, "hello");
}

#[test]
fn test_non_none_delimiter_group_in_attr_value() {
    paste! {
        #[cfg_attr(any(), doc = "text" (stuff))]
        struct NonNoneDelimiterGroupInAttrValue;
    }
    let _ = NonNoneDelimiterGroupInAttrValue;
}

#[test]
#[allow(unknown_lints, clippy::pedantic)]
fn test_none_group_before_double_colon_in_attr_context() {
    macro_rules! allow_lint {
        ($group:ty, $lint:ident) => {
            paste! {
                #[allow($group::$lint)]
                pub fn none_group_colon_colon_attr_fn() {}
            }
        };
    }
    allow_lint!(clippy, pedantic);
    none_group_colon_colon_attr_fn();
}
