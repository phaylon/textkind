#![cfg(feature = "serde")]

extern crate textkind;
extern crate serde;
extern crate serde_json;

#[test]
fn deserialize() {

    let text: textkind::Title<String> = serde_json::from_str("\"foo\"").unwrap();
    assert_eq!(text.as_str(), "foo");
}

#[test]
fn serialize() {

    let text = textkind::Title::<String>::try_from_str("foo").unwrap();
    let content = serde_json::to_string(&text).unwrap();
    assert_eq!(&content, "\"foo\"");
}

#[test]
fn deserialize_errors() {

    let result: Result<textkind::Title<String>, _> = serde_json::from_str("\"\"");
    let error = result.err().expect("empty string should fail");
    assert!(format!("{}", error).contains("invalid title"));
    assert!(format!("{}", error).contains("is empty"));
}
