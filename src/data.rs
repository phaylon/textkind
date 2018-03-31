
use std::borrow;

/// Data encapsulation value.
///
/// This is the type that allows `&'static str` values to avoid allocation.
///
/// The main advantage of dealing with `Data<T>` values is that static values can be
/// preserved for all dynamic storages, not just `String` as with `std::borrow::Cow`.
#[derive(Debug, Clone)]
pub enum Data<T> {
    /// Text is a static value.
    Static(&'static str),
    /// Text is a dynamic value.
    Dynamic(T),
}

impl<T> Data<T>
where
    T: ::Dynamic
{
    /// Create a data value from a `std::borrow::Cow<'static, str>`.
    pub fn from_static_str_cow(value: borrow::Cow<'static, str>) -> Data<T> {
        match value {
            borrow::Cow::Owned(value) => Data::Dynamic(T::from_string(value)),
            borrow::Cow::Borrowed(value) => Data::Static(value),
        }
    }

    /// Return a `&str` slice view into the stored text.
    pub fn as_str(&self) -> &str {
        match *self {
            Data::Static(value) => value,
            Data::Dynamic(ref dynamic) => dynamic.as_str(),
        }
    }

    /// Convert to another dynamic storage.
    pub fn convert<U>(self) -> Data<U>
    where
        U: ::Dynamic,
    {
        match self {
            Data::Static(value) => Data::Static(value),
            Data::Dynamic(dynamic) => Data::Dynamic(U::from(dynamic)),
        }
    }

    /// Turn the data value into a string, possibly extracting it without reallocating.
    pub fn into_string(self) -> String {
        match self {
            Data::Static(value) => value.into(),
            Data::Dynamic(dynamic) => dynamic.into_string(),
        }
    }

    /// Turn the data value into a `std::borrow::Cow<'static, str>`.
    pub fn into_static_str_cow(self) -> borrow::Cow<'static, str> {
        match self {
            Data::Static(value) => value.into(),
            Data::Dynamic(dynamic) => dynamic.into_string().into(),
        }
    }
}

