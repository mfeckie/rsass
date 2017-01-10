//! These are from the "basic" directory in the sass specification.
//! See https://github.com/sass/sass-spec for source material.
//! I add one a test function for one specification at a time and then
//! try to implement that functionality without breaking those already
//! added.
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn t01_simple_css() {
    check(b"a {\n  \
            color: blue;\n\
            }",
          b"a{color:blue}\n")
}

#[test]
fn t02_simple_nesting() {
    check(b"div {\n  img {\n    border: 0px;\n  }\n}",
          b"div img{border:0px}\n")
}

#[test]
fn t03_simple_variable() {
    check(b"$color: red;\n\na {\n  color: $color;\n}",
          b"a{color:red}\n")
}

#[test]
fn t04_basic_variables() {
    check(b"$color: \"black\";\n$color: red;\n$background: \"blue\";\n\n\
            a {\n  color: $color;\n  background: $background;\n}\n\n\
            $y: before;\n\n\
            $x: 1 2 $y;\n\n\
            foo {\n  a: $x;\n}\n\n\
            $y: after;\n\n\
            foo {\n  a: $x;\n}",
          b"a{color:red;background:\"blue\"}foo{a:1 2 before}\
            foo{a:1 2 before}\n")
}

fn check(input: &[u8], expected: &[u8]) {
    use std::str::from_utf8;
    let result = compile_scss(input, OutputStyle::Compressed);
    if let Ok(output) = result {
        if let (Ok(output), Ok(expected)) =
            (from_utf8(&output), from_utf8(expected)) {
            assert_eq!(output, expected)
        } else {
            assert_eq!(output, expected)
        }
    } else {
        assert_eq!(result, Ok(expected.into()))
    }
}