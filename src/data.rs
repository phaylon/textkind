
use std::borrow;

use small;

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
    /// Text is a small string fitting in an inline buffer.
    Small(small::SmallString),
}

impl<T> Data<T>
where
    T: ::Dynamic
{
    /// Create a static data value.
    pub fn from_static_str(value: &'static str) -> Data<T> {
        Data::Static(value)
    }

    /// Create a small or dynamic data value from a string slice.
    pub fn from_str(value: &str) -> Data<T> {
        match small::SmallString::try_from(value) {
            Some(small) => Data::Small(small),
            None => Data::Dynamic(T::from_str(value)),
        }
    }

    /// Create a dynamic data value from a string.
    pub fn from_string(value: String) -> Data<T> {
        Data::Dynamic(T::from_string(value))
    }

    /// Wrap an existing dynamic data storage.
    pub fn from_dynamic(value: T) -> Data<T> {
        Data::Dynamic(value)
    }

    /// Create a dynamic or small data value from a possibly owned value.
    pub fn from_cow(value: borrow::Cow<str>) -> Data<T> {
        match value {
            borrow::Cow::Owned(value) => Data::from_string(value),
            borrow::Cow::Borrowed(value) => Data::from_str(value),
        }
    }

    /// Create a data value from a `std::borrow::Cow<'static, str>`.
    pub fn from_static_str_cow(value: borrow::Cow<'static, str>) -> Data<T> {
        match value {
            borrow::Cow::Owned(value) => Data::from_string(value),
            borrow::Cow::Borrowed(value) => Data::from_static_str(value),
        }
    }

    /// Return a `&str` slice view into the stored text.
    pub fn as_str(&self) -> &str {
        match *self {
            Data::Static(value) => value,
            Data::Dynamic(ref dynamic) => dynamic.as_str(),
            Data::Small(ref small) => small.as_str(),
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
            Data::Small(small) => Data::Small(small),
        }
    }

    /// Turn the data value into a string, possibly extracting it without reallocating.
    pub fn into_string(self) -> String {
        match self {
            Data::Static(value) => value.into(),
            Data::Dynamic(dynamic) => dynamic.into_string(),
            Data::Small(small) => small.as_str().to_string(),
        }
    }

    /// Turn the data value into a `std::borrow::Cow<'static, str>`.
    pub fn into_static_str_cow(self) -> borrow::Cow<'static, str> {
        match self {
            Data::Static(value) => value.into(),
            Data::Dynamic(dynamic) => dynamic.into_string().into(),
            Data::Small(small) => small.as_str().to_string().into(),
        }
    }

    /// Turn the data value into a dynamic storage, possible simply unwrapping.
    pub fn into_dynamic(self) -> T {
        match self {
            Data::Static(value) => T::from_str(value),
            Data::Dynamic(value) => value,
            Data::Small(value) => T::from_str(value.as_str()),
        }
    }

    /// Check if data is a static str value.
    pub fn is_static(&self) -> bool {
        if let Data::Static(_) = *self {
            true
        } else {
            false
        }
    }

    /// Check if data is a dynamic storage value.
    pub fn is_dynamic(&self) -> bool {
        if let Data::Dynamic(_) = *self {
            true
        } else {
            false
        }
    }

    /// Check if data is a small string value.
    pub fn is_small(&self) -> bool {
        if let Data::Small(_) = *self {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sso() {
        assert!(Data::<String>::from_str("").is_small());
        assert!(Data::<String>::from_str(&"X".repeat(16)).is_small());
        assert!(Data::<String>::from_str(&"X".repeat(17)).is_dynamic());
    }

    #[test]
    fn static_construction() {
        assert!(Data::<String>::from_static_str("foo").is_static());
    }
}
