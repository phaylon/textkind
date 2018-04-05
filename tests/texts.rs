
extern crate textkind;

use textkind::*;

struct TestKind;

impl ::Kind for TestKind {

    type Check = ::check::NotEmpty;

    const DESCRIPTION: &'static str = "test";
}

macro_rules! test_storage_transition {
    ($dynamic:ty: $( $other_name:ident: $other_dynamic:ty ),* $(,)*) => {
        $(
            mod $other_name {
                use super::*;

                #[test]
                fn storage_transition() {
                    let text = Text::<TestKind, $dynamic>::try_from_str("foo").unwrap();
                    let other: Text<_, $other_dynamic> = text
                        .storage_transition();
                    assert_eq!(other.as_str(), "foo");
                }
            }
        )*
    }
}

macro_rules! text_tests {
    ($name:ident: $dynamic:ty) => {

        mod $name {
            use super::*;

            type Test = Text<super::TestKind, $dynamic>;

            #[test]
            fn try_from_static_str() {

                let text = Test::try_from_static_str("foo")
                    .expect("valid value");
                assert_eq!(text.as_str(), "foo");

                let error = Test::try_from_static_str("")
                    .err()
                    .expect("invalid value");
                assert!(format!("{:?}", error).contains("NotEmptyError"));
            }

            #[test]
            fn try_from_str() {

                let string = "foo".to_string();
                let text = Test::try_from_str(&string)
                    .expect("valid value");
                assert_eq!(text.as_str(), "foo");

                let string = String::new();
                let error = Test::try_from_str(&string)
                    .err()
                    .expect("invalid value");
                assert!(format!("{:?}", error).contains("NotEmptyError"));
            }

            #[test]
            fn try_from_str_cow_owned() {

                let string = "foo".to_string();
                let text = Test::try_from_str_cow(string.into())
                    .expect("valid value");
                assert_eq!(text.as_str(), "foo");

                let string = String::new();
                let error = Test::try_from_str_cow(string.into())
                    .err()
                    .expect("invalid value");
                assert!(format!("{:?}", error).contains("NotEmptyError"));
                assert_eq!(error.value(), "");
            }

            #[test]
            fn try_from_str_cow_borrowed() {

                let string = "foo".to_string();
                let text = Test::try_from_str_cow(::std::borrow::Cow::Borrowed(&string))
                    .expect("valid value");
                assert_eq!(text.as_str(), "foo");

                let string = String::new();
                let error = Test::try_from_str_cow(::std::borrow::Cow::Borrowed(&string))
                    .err()
                    .expect("invalid value");
                assert!(format!("{:?}", error).contains("NotEmptyError"));
                assert_eq!(error.value(), "");
            }

            #[test]
            fn try_from_static_str_cow_owned() {

                let string = "foo".to_string();
                let text = Test::try_from_static_str_cow(string.into())
                    .expect("valid value");
                assert_eq!(text.as_str(), "foo");

                let string = String::new();
                let error = Test::try_from_static_str_cow(string.into())
                    .err()
                    .expect("invalid value");
                assert!(format!("{:?}", error).contains("NotEmptyError"));
                assert_eq!(error.value(), "");
            }

            #[test]
            fn try_from_static_str_cow_borrowed() {

                let text = Test::try_from_static_str_cow("foo".into())
                    .expect("valid value");
                assert_eq!(text.as_str(), "foo");

                let error = Test::try_from_static_str_cow("".into())
                    .err()
                    .expect("invalid value");
                assert!(format!("{:?}", error).contains("NotEmptyError"));
                assert_eq!(error.value(), "");
            }

            #[test]
            fn try_from_string() {

                let string = "foo".to_string();
                let text = Test::try_from_string(string)
                    .expect("valid value");
                assert_eq!(text.as_str(), "foo");

                let error = Test::try_from_string(String::new())
                    .err()
                    .expect("invalid value");
                assert!(format!("{:?}", error).contains("NotEmptyError"));
                assert_eq!(error.value(), "");
            }

            #[test]
            fn try_from_data() {
                use Dynamic;

                let dynamic = Dynamic::from_str("foo");
                let text = Test::try_from_data(Data::Dynamic(dynamic))
                    .expect("valid value");
                assert_eq!(text.as_str(), "foo");

                let error = Test::try_from_data(Data::Dynamic(Dynamic::from_str("")))
                    .err()
                    .expect("invalid value");
                assert!(format!("{:?}", error).contains("NotEmptyError"));
                assert_eq!(error.value().as_str(), "");
            }

            #[test]
            fn try_from_dynamic() {
                use Dynamic;

                let dynamic = Dynamic::from_str("foo");
                let text = Test::try_from_dynamic(dynamic)
                    .expect("valid value");
                assert_eq!(text.as_str(), "foo");

                let error = Test::try_from_dynamic(Dynamic::from_str(""))
                    .err()
                    .expect("invalid value");
                assert!(format!("{:?}", error).contains("NotEmptyError"));
                assert_eq!(error.value().as_str(), "");
            }

            #[test]
            fn try_convert_from() {

                struct OtherKind;

                impl Kind for OtherKind {

                    type Check = ::check::NoControl;

                    const DESCRIPTION: &'static str = "other";
                }

                impl TryConvertFrom<OtherKind> for TestKind {

                    type Error = Error<TestKind>;

                    fn try_convert_from<D>(other: Text<OtherKind, D>)
                    -> ConvertResult<OtherKind, TestKind, D, Error<TestKind>>
                    where
                        D: Dynamic,
                    {
                        match other.try_kind_transition() {
                            Ok(value) => Ok(value),
                            Err(error) => Err(error.into()),
                        }
                    }
                }

                let other = Text::<OtherKind, $dynamic>::try_from_str("foo").unwrap();
                let text = match Test::try_convert_from(other) {
                    Ok(text) => text,
                    Err(ConvertError(Error(::check::NotEmptyError), _value)) =>
                        panic!("value unexpectedly invalid"),
                };
                assert_eq!(text.as_str(), "foo");

                let other = Text::<OtherKind, $dynamic>::try_from_str("").unwrap();
                match Test::try_convert_from(other) {
                    Ok(_text) => panic!("unexpectedly valid"),
                    Err(ConvertError(Error(::check::NotEmptyError), _value)) => (),
                };
            }

            #[test]
            fn try_convert_into() {

                struct OtherKind;

                impl Kind for OtherKind {

                    type Check = ::check::NoControl;

                    const DESCRIPTION: &'static str = "other";
                }

                impl TryConvertFrom<OtherKind> for TestKind {

                    type Error = Error<Self>;

                    fn try_convert_from<D>(other: Text<OtherKind, D>)
                    -> ConvertResult<OtherKind, TestKind, D, Error<TestKind>>
                    where
                        D: Dynamic,
                    {
                        match other.try_kind_transition() {
                            Ok(value) => Ok(value),
                            Err(error) => Err(error.into()),
                        }
                    }
                }

                let other = Text::<OtherKind, $dynamic>::try_from_str("foo").unwrap();
                let text = match other.try_convert_into::<TestKind>() {
                    Ok(text) => text,
                    Err(ConvertError(Error(::check::NotEmptyError), _value)) =>
                        panic!("value unexpectedly invalid"),
                };
                assert_eq!(text.as_str(), "foo");

                let other = Text::<OtherKind, $dynamic>::try_from_str("").unwrap();
                match other.try_convert_into::<TestKind>() {
                    Ok(_text) => panic!("unexpectedly valid"),
                    Err(ConvertError(Error(::check::NotEmptyError), _value)) => (),
                };
            }

            #[test]
            fn convert_from() {

                struct OtherKind;

                impl Kind for OtherKind {

                    type Check = ::check::NotEmpty;

                    const DESCRIPTION: &'static str = "other";
                }

                impl ConvertFrom<OtherKind> for TestKind {

                    fn convert_from<D>(other: Text<OtherKind, D>) -> Text<TestKind, D>
                    where
                        D: Dynamic,
                    {
                        match other.try_kind_transition() {
                            Ok(value) => value,
                            _ => panic!("kind transition failed"),
                        }
                    }
                }

                let other = Text::<OtherKind, $dynamic>::try_from_str("foo").unwrap();
                let text = Test::convert_from(other);
                assert_eq!(text.as_str(), "foo");
            }

            #[test]
            fn convert_into() {

                struct OtherKind;

                impl Kind for OtherKind {

                    type Check = ::check::NotEmpty;

                    const DESCRIPTION: &'static str = "other";
                }

                impl ConvertFrom<TestKind> for OtherKind {

                    fn convert_from<D>(other: Text<TestKind, D>) -> Text<OtherKind, D>
                    where
                        D: Dynamic,
                    {
                        match other.try_kind_transition() {
                            Ok(value) => value,
                            _ => panic!("kind transition failed"),
                        }
                    }
                }

                let text = Test::try_from_str("foo").unwrap();
                let other: Text<OtherKind, _> = text.convert_into();
                assert_eq!(other.as_str(), "foo");
            }

            #[test]
            fn into_string() {

                let text = Test::try_from_str("foo").unwrap();
                let string = text.into_string();
                assert_eq!(&string, "foo");
            }

            #[test]
            fn into_static_str_cow() {

                let text = Test::try_from_str("foo").unwrap();
                let cow = text.into_static_str_cow();
                if let ::std::borrow::Cow::Borrowed(_) = cow {
                    panic!("borrowed instead of owned");
                }

                let text = Test::try_from_static_str("foo").unwrap();
                let cow = text.into_static_str_cow();
                if let ::std::borrow::Cow::Owned(_) = cow {
                    panic!("owned instead of borrowed");
                }
            }

            #[test]
            fn into_data() {

                let text = Test::try_from_static_str("foo").unwrap();
                let data = text.into_data();
                assert_eq!(data.as_str(), "foo");
                match data {
                    Data::Static(_) => (),
                    _ => panic!("non static data variant"),
                }
            }

            #[test]
            fn into_dynamic() {

                let text = Test::try_from_str("foo").unwrap();
                let string = text.into_dynamic();
                assert_eq!(string.as_str(), "foo");
            }

            #[test]
            fn try_kind_transition() {

                struct OtherKind;

                impl Kind for OtherKind {

                    type Check = ::check::SingleLine;

                    const DESCRIPTION: &'static str = "other";
                }

                let text = Test::try_from_str("foo").unwrap();
                let _: Text<OtherKind, _> = text
                    .try_kind_transition()
                    .expect("kind transition");

                let text = Test::try_from_str("foo\nbar").unwrap();
                let result: Result<Text<OtherKind, _>, _> = text
                    .try_kind_transition();
                let error = result.err().expect("error result");
                let _: &Test = error.value();
            }

            #[test]
            fn kind_transition() {

                struct OtherKind;

                impl Kind for OtherKind {

                    type Check = ::check::NotEmpty;

                    const DESCRIPTION: &'static str = "other";
                }

                let text = Test::try_from_str("foo").unwrap();
                let other: Text<OtherKind, _> = text.kind_transition();
                assert_eq!(other.as_str(), "foo");
            }

            test_storage_transition! {
                $dynamic:
                storage_transition_string: String,
                storage_transition_arc_string: ::std::sync::Arc<String>,
                storage_transition_rc_string: ::std::rc::Rc<String>,
            }
        }
    }
}

text_tests!(string: String);
text_tests!(rc_string: ::std::rc::Rc<String>);
text_tests!(arc_string: ::std::sync::Arc<String>);

#[test]
fn title() {

    for valid in &["This is a title.", "X"] {
        let text = Title::<String>::try_from_str(valid)
            .expect("valid value");
        assert_eq!(text.as_str(), *valid);
    }

    let long = "X".repeat(513);
    for invalid in &["", "Foo\nBar", " Foo", "Foo ", " Foo ", &long] {
        Title::<String>::try_from_str(invalid)
            .err()
            .expect("invalid value");
    }
}

#[test]
fn identifier() {

    for valid in &["foo", "Foo_Bar", "_23", "_"] {
        let text = Identifier::<String>::try_from_str(valid)
            .expect("valid value");
        assert_eq!(text.as_str(), *valid);
    }

    let long = "X".repeat(513);
    for invalid in &["", " ", "a b", "a\nb", "a-b", "-", "0", &long] {
        Identifier::<String>::try_from_str(invalid)
            .err()
            .expect("invalid value");
    }
}

#[test]
fn identifier_lax() {

    for valid in &["foo", "Foo_Bar", "_23", "_", "-", "0"] {
        let text = IdentifierLax::<String>::try_from_str(valid)
            .expect("valid value");
        assert_eq!(text.as_str(), *valid);
    }

    let long = "X".repeat(513);
    for invalid in &["", " ", "a b", "a\nb", &long] {
        IdentifierLax::<String>::try_from_str(invalid)
            .err()
            .expect("invalid value");
    }
}

#[test]
fn modified() {

    let modified: Modified<String> = "foo".to_string().into();
    assert_eq!(modified, Modified::New("foo".to_string()));

    let modified: Modified<String> = "foo".into();
    assert_eq!(modified, Modified::Sub("foo"));
}

#[test]
fn clone() {

    let text = Title::<String>::try_from_str("foo").unwrap();
    let text2 = text.clone();
    assert_eq!(text, text2);
}

#[test]
fn from_str() {

    let _: Title<String> = "foo".parse().expect("valid parse");

    let result: Result<Title<String>, _> = "foo\nbar".parse();
    assert!(result.is_err());
}

#[test]
fn debug() {

    let text = Title::<String>::try_from_str("foo").unwrap();
    assert!(format!("{:?}", text).starts_with("Text { data: "));
    assert!(format!("{:?}", text).ends_with(" }"));
}

#[test]
fn display() {

    let text = Title::<String>::try_from_str("foo").unwrap();
    assert_eq!(&format!("{}", text), "foo");
}

#[test]
fn eq() {
    
    let text = Title::<String>::try_from_str("foo").unwrap();
    let text2 = Title::<String>::try_from_str("foo").unwrap();
    let text_diff = Title::<String>::try_from_str("bar").unwrap();
    assert_eq!(text, text2);
    assert_ne!(text, text_diff);
}

#[test]
fn ord() {

    let a = Title::<String>::try_from_str("a").unwrap();
    let b = Title::<String>::try_from_str("b").unwrap();
    assert!(a < b);
    assert!(b > a);
}

#[test]
fn hash() {

    let mut set = ::std::collections::HashSet::new();
    let a = Title::<String>::try_from_str("foo").unwrap();
    let b = Title::<String>::try_from_str("bar").unwrap();

    set.insert(a.clone());
    assert!(set.get(&a).is_some());
    assert!(set.get(&b).is_none());
}

#[test]
fn as_ref() {
    
    let text = Title::<String>::try_from_str("foo").unwrap();
    let slice: &str = text.as_ref();
    assert_eq!(slice, "foo");
}

#[test]
fn deref() {

    let text = Title::<String>::try_from_str("foo").unwrap();
    let slice: &str = &text;
    assert_eq!(slice, "foo");
}

