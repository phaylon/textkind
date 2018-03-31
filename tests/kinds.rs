
extern crate textkind;

use textkind::kind::*;

macro_rules! assert_ok {
    ($kind:ident: $value:expr) => {
        match textkind::Text::<$kind, String>::try_from_str($value) {
            Ok(_) => (),
            Err(error) => panic!(
                "kind {} failed for value {:?} with {:?}",
                stringify!($kind),
                $value,
                error
            ),
        }
    }
}

macro_rules! assert_err {
    ($kind:ident: $value:expr) => {
        match textkind::Text::<$kind, String>::try_from_str($value) {
            Err(_) => (),
            Ok(_) => panic!(
                "kind {} unexpectedly accepts value {:?} as valid",
                stringify!($kind),
                $value,
            ),
        }
    }
}

#[test]
fn title() {

    assert_ok!(Title: "This is a title.");
    assert_ok!(Title: "X");

    assert_err!(Title: "");
    assert_err!(Title: "Foo\nBar");
    assert_err!(Title: "  Foo");
    assert_err!(Title: "Foo  ");
    assert_err!(Title: " Foo ");
    assert_err!(Title: &"X".repeat(513));
}

#[test]
fn identifier() {

    assert_ok!(Identifier: "foo");
    assert_ok!(Identifier: "foo_bar");
    assert_ok!(Identifier: "_23");
    assert_ok!(Identifier: "_");

    assert_err!(Identifier: "");
    assert_err!(Identifier: " ");
    assert_err!(Identifier: "foo bar");
    assert_err!(Identifier: "foo\nbar");
    assert_err!(Identifier: "foo-bar");
    assert_err!(Identifier: "-");
    assert_err!(Identifier: "0");
}

#[test]
fn identifier_lax() {

    assert_ok!(IdentifierLax: "foo");
    assert_ok!(IdentifierLax: "foo_bar");
    assert_ok!(IdentifierLax: "_23");
    assert_ok!(IdentifierLax: "_");
    assert_ok!(IdentifierLax: "foo-bar");
    assert_ok!(IdentifierLax: "-");
    assert_ok!(IdentifierLax: "0");

    assert_err!(IdentifierLax: "");
    assert_err!(IdentifierLax: " ");
    assert_err!(IdentifierLax: "foo bar");
    assert_err!(IdentifierLax: "foo\nbar");
}

