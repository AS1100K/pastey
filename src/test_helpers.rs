// These functions are used to test the `expand_attr` and `parse_bracket_as_segments` functions,
// Which can not be called directly from the test suite because they are not public API.
// and also as unit test also can not be used due to the `proc_macro` crate's limitations in unit tests.
// Coverage is off for these functions because these are just helper function for testing,
// and we are not using in production code, so we don't need to cover them.

use crate::attr::expand_attr;
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use std::str::FromStr;

use super::{expand, parse_bracket_as_segments, pasted_to_tokens};

#[cfg_attr(coverage_nightly, coverage(off))]
pub(super) fn expand_attr_test(scope: Span) {
    {
        let mut attr_ts = TokenStream::new();
        attr_ts.extend([
            TokenTree::Ident(Ident::new("doc", scope)),
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::None, TokenStream::new())),
        ]);
        let mut flag = false;
        let _ = expand_attr(attr_ts, scope, &mut flag);
    }

    {
        let mut attr_ts = TokenStream::new();
        attr_ts.extend([
            TokenTree::Ident(Ident::new("doc", scope)),
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),
            TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
            TokenTree::Punct(Punct::new('\'', Spacing::Alone)),
        ]);
        let _ = expand_attr(attr_ts, scope, &mut false);
    }

    {
        let mut attr_ts = TokenStream::new();
        attr_ts.extend([
            TokenTree::Ident(Ident::new("allow", scope)),
            TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                TokenStream::from_str("doc = : \"world\"").unwrap(),
            )),
        ]);
        let _ = expand_attr(attr_ts, scope, &mut false);
    }

    {
        let mut paren_ts = TokenStream::from_str("doc = : \"world\"").unwrap();
        paren_ts.extend([
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            TokenTree::Ident(Ident::new("allow", scope)),
        ]);
        let mut attr_ts = TokenStream::new();
        attr_ts.extend([
            TokenTree::Ident(Ident::new("cfg_attr", scope)),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, paren_ts)),
        ]);
        let _ = expand_attr(attr_ts, scope, &mut false);
    }

    {
        let mut contains = false;
        let _ = expand(
            TokenStream::from_str("# [ doc = : \"world\" ] fn f () { }").unwrap(),
            &mut contains,
            true,
        );
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
pub(super) fn parse_bracket_as_segments_test(scope: Span) {
    let _ = parse_bracket_as_segments(TokenStream::from_str("foo >").unwrap(), scope);
    let _ = parse_bracket_as_segments(TokenStream::new(), scope);
    let _ = parse_bracket_as_segments(TokenStream::from_str("< foo").unwrap(), scope);
    let _ = parse_bracket_as_segments(TokenStream::from_str("< foo > extra").unwrap(), scope);
    let _ = parse_bracket_as_segments(TokenStream::from_str("< foo +").unwrap(), scope);
    let _ = pasted_to_tokens(String::from("0invalid"), scope);
    let _ = pasted_to_tokens(String::from("0 "), scope);
    let _ = parse_bracket_as_segments(TokenStream::from_str("< env !").unwrap(), scope);

    {
        let mut inner_ts = TokenStream::new();
        inner_ts.extend([TokenTree::Punct(Punct::new('@', Spacing::Alone))]);
        let mut ts = TokenStream::new();
        ts.extend([
            TokenTree::Punct(Punct::new('<', Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::None, inner_ts)),
            TokenTree::Punct(Punct::new('>', Spacing::Alone)),
        ]);
        let _ = parse_bracket_as_segments(ts, scope);
    }
}
