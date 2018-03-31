
extern crate textkind;

use textkind::*;

#[test]
fn error_with_value() {
    use std::error::{Error};

    let error = Title::<String>::try_from_string("a\nb".into())
        .err()
        .expect("invalid value");
    assert_eq!(error.value(), "a\nb");
    let msg = format!("{}", error);
    assert_eq!(&format!("{}", error), "invalid title");
    assert!(format!("{:?}", error).contains("NoControlError"));
    assert!(format!("{:?}", error).contains("ErrorWithValue"));
    assert!(!format!("{}", error).contains("control"));
    assert!(format!("{}", error.cause().expect("check in cause")).contains("control"));

    let error_without = error.clone().without_value();
    assert_eq!(msg, format!("{}", error_without));

    let (error_without, value) = error.clone().split();
    assert_eq!(msg, format!("{}", error_without));
    assert_eq!(&value, "a\nb");

    let mapped = error.clone().map_value(|value| value.repeat(2));
    assert_eq!(msg, format!("{}", error_without));
    assert_eq!(mapped.value(), "a\nba\nb");

    let other_error = Title::<String>::try_from_string("a\nb".into())
        .err()
        .expect("invalid value");
    assert_eq!(error, other_error);

    let other_error_diff = Title::<String>::try_from_string("a\nbc".into())
        .err()
        .expect("invalid value");
    assert_ne!(error, other_error_diff);
}

#[test]
fn error() {
    use std::error::{Error};

    let error = Title::<String>::try_from_str("a\nb")
        .err()
        .expect("invalid value");
    assert_eq!(&format!("{}", error), "invalid title");
    assert!(format!("{:?}", error).contains("NoControlError"));
    assert!(!format!("{}", error).contains("control"));
    assert!(format!("{}", error.cause().expect("check in cause")).contains("control"));

    let other_error = Title::<String>::try_from_str("a\nb".into())
        .err()
        .expect("invalid value");
    assert_eq!(error, other_error);

    let other_error_diff = Title::<String>::try_from_str("a\nbc".into())
        .err()
        .expect("invalid value");
    assert_eq!(error, other_error_diff);

    let with_value = error.clone().with_value("foo");
    assert_eq!(*with_value.value(), "foo");
    assert_eq!(&format!("{}", with_value), "invalid title");
    assert!(format!("{:?}", with_value).contains("NoControlError"));
    assert!(format!("{:?}", with_value).contains("ErrorWithValue"));
    assert!(!format!("{}", with_value).contains("control"));
    assert!(format!("{}", with_value.cause().expect("check in cause")).contains("control"));
}

