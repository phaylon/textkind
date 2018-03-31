
extern crate textkind;

use textkind::check::*;

macro_rules! expect_pass {
    ($check:ty: $value:expr) => {
        <$check as textkind::Check>::check($value).expect(&format!(
            "expected {} to consider {:?} valid",
            stringify!($check),
            $value,
        ))
    }
}

macro_rules! expect_fail {
    ($check:ty: $value:expr) => {
        <$check as textkind::Check>::check($value).err().expect(&format!(
            "expected {} to consider {:?} invalid",
            stringify!($check),
            $value,
        ))
    }
}

macro_rules! assert_display {
    ($value:expr, $search:expr) => {
        assert!(
            format!("{}", $value).contains($search),
            "std::fmt::Display output {:?} does not contain {:?}",
            $value,
            $search
        )
    }
}

macro_rules! assert_debug {
    ($value:expr, $search:expr) => {
        assert!(
            format!("{:?}", $value).contains($search),
            "std::fmt::Debug output {:?} does not contain {:?}",
            $value,
            $search
        )
    }
}

#[test]
fn not_empty() {

    expect_pass!(NotEmpty: "foo");
    expect_pass!(NotEmpty: " ");
    expect_pass!(NotEmpty: "\n");

    let error = expect_fail!(NotEmpty: "");
    assert_display!(error, "is empty");
    assert_debug!(error, "NotEmptyError");
}

#[test]
fn single_line() {

    expect_pass!(SingleLine: "foo");
    expect_pass!(SingleLine: "");
    expect_pass!(SingleLine: " ");

    let error = expect_fail!(SingleLine: "foo\nbar");
    assert_display!(error, "single-line");
    assert_debug!(error, "SingleLineError");

    expect_fail!(SingleLine: "\n");
    expect_fail!(SingleLine: "\n\n");
}

#[test]
fn no_whitespace() {

    expect_pass!(NoWhitespace: "foo");
    expect_pass!(NoWhitespace: "");

    let error = expect_fail!(NoWhitespace: "foo bar baz");
    assert_display!(error, "2 whitespace");
    assert_debug!(error, "NoWhitespaceError");

    expect_fail!(NoWhitespace: " ");
    expect_fail!(NoWhitespace: "\n");
    expect_fail!(NoWhitespace: "\t");
}

#[test]
fn no_control() {
    
    expect_pass!(NoControl: "foo");
    expect_pass!(NoControl: "");
    expect_pass!(NoControl: " ");
    
    let error = expect_fail!(NoControl: "foo\nbar\tbaz");
    assert_display!(error, "2 control character");
    assert_debug!(error, "NoControlError");
    
    expect_fail!(NoControl: "\r");
}

#[test]
fn when_trimmed() {

    type TestCheck = WhenTrimmed<NotEmpty>;

    expect_pass!(TestCheck: "foo");

    let error = expect_fail!(TestCheck: "  \n");
    assert_display!(error, "is empty");
    assert_display!(error, "when trimmed");
    assert_debug!(error, "WhenTrimmedError");
    assert_debug!(error, "NotEmptyError");

    expect_fail!(TestCheck: " ");
    expect_fail!(TestCheck: "\n");
    expect_fail!(TestCheck: "\t");
    expect_fail!(TestCheck: "");
}

#[test]
fn and() {

    type TestCheck = And<SingleLine, NoWhitespace>;

    expect_pass!(TestCheck: "foo");

    let error = expect_fail!(TestCheck: "foo bar");
    assert_display!(error, "1 whitespace");
    assert_debug!(error, "NoWhitespaceError");

    let error = expect_fail!(TestCheck: "foo\nbar");
    assert_display!(error, "single-line");
    assert_debug!(error, "SingleLineError");

    let error = expect_fail!(TestCheck: "foo \n bar");
    assert_debug!(error, "SingleLineError");
}

#[test]
fn trimmed_left() {

    expect_pass!(TrimmedLeft: "foo");
    expect_pass!(TrimmedLeft: "foo  ");
    expect_pass!(TrimmedLeft: "");

    let error = expect_fail!(TrimmedLeft: "  foo");
    assert_display!(error, "whitespace");
    assert_display!(error, "at the end");
    assert_debug!(error, "TrimmedLeftError");

    expect_fail!(TrimmedLeft: " ");
    expect_fail!(TrimmedLeft: "\n");
    expect_fail!(TrimmedLeft: "\t");
}

#[test]
fn trimmed_right() {

    expect_pass!(TrimmedRight: "foo");
    expect_pass!(TrimmedRight: "  foo");
    expect_pass!(TrimmedRight: "");

    let error = expect_fail!(TrimmedRight: "foo  ");
    assert_display!(error, "whitespace");
    assert_display!(error, "at the beginning");
    assert_debug!(error, "TrimmedRightError");

    expect_fail!(TrimmedRight: " ");
    expect_fail!(TrimmedRight: "\n");
    expect_fail!(TrimmedRight: "\t");
}

#[test]
fn trimmed() {

    expect_pass!(Trimmed: "foo");
    expect_pass!(Trimmed: "");

    let error = expect_fail!(Trimmed: "foo  ");
    assert_debug!(error, "TrimmedRightError");

    let error = expect_fail!(Trimmed: "  foo");
    assert_debug!(error, "TrimmedLeftError");

    let error = expect_fail!(Trimmed: "  foo  ");
    assert_debug!(error, "TrimmedBothError");
    assert_display!(error, "whitespace");
    assert_display!(error, "beginning and end");

    let error = expect_fail!(Trimmed: "  ");
    assert_debug!(error, "TrimmedOnlyError");
    assert_display!(error, "only whitespace");
    
    expect_fail!(Trimmed: "\n");
    expect_fail!(Trimmed: "\t");
}

#[test]
fn identifier() {

    expect_pass!(Identifier: "foo");
    expect_pass!(Identifier: "foo23");
    expect_pass!(Identifier: "foo_23");
    expect_pass!(Identifier: "_foo");
    expect_pass!(Identifier: "_23");
    expect_pass!(Identifier: "f");
    expect_pass!(Identifier: "_");

    let error = expect_fail!(Identifier: "");
    assert_debug!(error, "NotEmptyError");

    let error = expect_fail!(Identifier: "foo bar");
    assert_debug!(error, "InvalidRestChar");
    assert_display!(error, "` `");

    let error = expect_fail!(Identifier: "-foo");
    assert_debug!(error, "InvalidStartChar");
    assert_display!(error, "`-`");

    let error = expect_fail!(Identifier: "0foo");
    assert_debug!(error, "InvalidStartChar");
    assert_display!(error, "`0`");

    let error = expect_fail!(Identifier: "-");
    assert_debug!(error, "InvalidStartChar");
    assert_display!(error, "`-`");

    let error = expect_fail!(Identifier: "0");
    assert_debug!(error, "InvalidStartChar");
    assert_display!(error, "`0`");

    let error = expect_fail!(Identifier: "foo-bar");
    assert_debug!(error, "InvalidRestChar");
    assert_display!(error, "`-`");
}

#[test]
fn identifier_lax() {
    
    expect_pass!(IdentifierLax: "foo");
    expect_pass!(IdentifierLax: "foo23");
    expect_pass!(IdentifierLax: "foo_23");
    expect_pass!(IdentifierLax: "_foo");
    expect_pass!(IdentifierLax: "_23");
    expect_pass!(IdentifierLax: "f");
    expect_pass!(IdentifierLax: "_");
    expect_pass!(IdentifierLax: "foo-bar");
    expect_pass!(IdentifierLax: "-");
    expect_pass!(IdentifierLax: "0");

    let error = expect_fail!(IdentifierLax: "");
    assert_debug!(error, "NotEmptyError");

    let error = expect_fail!(IdentifierLax: "foo bar");
    assert_debug!(error, "InvalidChar");
    assert_display!(error, "` `");
}

#[test]
fn title() {

    expect_pass!(Title: "Foo Bar: The Baz Story");

    let error = expect_fail!(Title: "");
    assert_debug!(error, "NotEmptyError");

    let error = expect_fail!(Title: "Foo\nBar");
    assert_debug!(error, "NoControlError");

    let error = expect_fail!(Title: "Foo  ");
    assert_debug!(error, "TrimmedRightError");

    let error = expect_fail!(Title: "  Foo");
    assert_debug!(error, "TrimmedLeftError");

    let error = expect_fail!(Title: "  Foo  ");
    assert_debug!(error, "TrimmedBothError");
}

#[test]
fn max_bytes() {

    expect_pass!(MaxBytes256: "foo");
    expect_pass!(MaxBytes256: "");
    expect_pass!(MaxBytes256: &"X".repeat(256));

    expect_pass!(MaxBytes512: "foo");
    expect_pass!(MaxBytes512: "");
    expect_pass!(MaxBytes512: &"X".repeat(512));

    expect_pass!(MaxBytes1024: "foo");
    expect_pass!(MaxBytes1024: "");
    expect_pass!(MaxBytes1024: &"X".repeat(1024));

    let error = expect_fail!(MaxBytes256: &"X".repeat(257));
    assert_display!(error, "length of 257");
    assert_display!(error, "limit of 256");
    assert_debug!(error, "MaxBytesError");

    let error = expect_fail!(MaxBytes512: &"X".repeat(513));
    assert_display!(error, "length of 513");
    assert_display!(error, "limit of 512");
    assert_debug!(error, "MaxBytesError");

    let error = expect_fail!(MaxBytes1024: &"X".repeat(1025));
    assert_display!(error, "length of 1025");
    assert_display!(error, "limit of 1024");
    assert_debug!(error, "MaxBytesError");
}

