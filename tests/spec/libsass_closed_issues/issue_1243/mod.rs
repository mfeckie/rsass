//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1243/at-rule.hrx"
#[test]
fn at_rule() {
    assert_eq!(
        rsass(
            "@foo bar\
            \n"
        )
        .unwrap(),
        "@foo bar;\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1243/comma-list.hrx"
#[test]
fn comma_list() {
    assert_eq!(
        rsass(
            "$a: 1, 2\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1243/debug.hrx"
#[test]
fn debug() {
    assert_eq!(
        rsass(
            "@debug(\"foo\")\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1243/import.hrx"
#[test]
fn import() {
    assert_eq!(
        rsass(
            "@import \"http://foo\"\
            \n"
        )
        .unwrap(),
        "@import \"http://foo\";\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1243/include.hrx"
#[test]
fn include() {
    assert_eq!(
        rsass(
            "@mixin foo() {\
            \n  a { b: c; }\
            \n}\
            \n@include foo\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1243/space-list.hrx"
#[test]
fn space_list() {
    assert_eq!(
        rsass(
            "$a: 1 2\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1243/warning.hrx"
#[test]
fn warning() {
    assert_eq!(
        rsass(
            "@warning \"foo\"\
            \n"
        )
        .unwrap(),
        "@warning \"foo\";\
        \n"
    );
}
