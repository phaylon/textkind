
use std::error;
use std::fmt;

/// An error with an associated value.
///
/// This error will be returned when the validation of an owned value failed. It allows
/// the owned value to be accessed or extracted.
///
/// It is a tuple containing the actual error as defined by the `Check` type of the `Kind`
/// type parameter, and the value.
pub struct ErrorWithValue<K, V>(
    pub <<K as ::Kind>::Check as ::Check>::Error,
    pub V,
) where
    K: ::Kind;

impl<K, V> ErrorWithValue<K, V>
where
    K: ::Kind,
{
    /// Discard the value and return an `Error`, which is the error variant without an
    /// associated value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate textkind;
    ///
    /// let input = "invalid\ntitle".to_string();
    /// let error_with_value =
    ///     textkind::Title::<String>::try_from_string(input)
    ///     .err()
    ///     .expect("input with control characters is not a valid title");
    ///
    /// let error_without_value = error_with_value.without_value();
    /// ```
    pub fn without_value(self) -> Error<K> { Error(self.0) }

    /// Extract the value into a tuple containing an error without a value, and the extracted
    /// value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate textkind;
    ///
    /// let input = "invalid\ntitle".to_string();
    /// let error_with_value =
    ///     textkind::Title::<String>::try_from_string(input)
    ///     .err()
    ///     .expect("input with control characters is not a valid title");
    ///
    /// let (error_without_value, value) = error_with_value.split();
    /// ```
    pub fn split(self) -> (Error<K>, V) { (Error(self.0), self.1) }

    /// Access the value associated with the error.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate textkind;
    ///
    /// let input = "invalid\ntitle".to_string();
    /// let error_with_value =
    ///     textkind::Title::<String>::try_from_string(input)
    ///     .err()
    ///     .expect("input with control characters is not a valid title");
    ///
    /// assert_eq!(error_with_value.value(), "invalid\ntitle");
    /// ```
    pub fn value(&self) -> &V { &self.1 }

    /// Map the associated value to another type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate textkind;
    ///
    /// let input = "invalid\ntitle".to_string();
    /// let error_with_value =
    ///     textkind::Title::<String>::try_from_string(input)
    ///     .err()
    ///     .expect("input with control characters is not a valid title");
    ///
    /// assert_eq!(
    ///     error_with_value
    ///         .map_value(|value| value.to_uppercase())
    ///         .value(),
    ///     "INVALID\nTITLE",
    /// )
    /// ```
    pub fn map_value<V2, F>(self, map: F) -> ErrorWithValue<K, V2>
    where
        F: FnOnce(V) -> V2,
    {
        ErrorWithValue(self.0, map(self.1))
    }
}

impl<K, V> Clone for ErrorWithValue<K, V>
where
    K: ::Kind,
    <<K as ::Kind>::Check as ::Check>::Error: Clone,
    V: Clone,
{
    fn clone(&self) -> Self { ErrorWithValue(self.0.clone(), self.1.clone()) }
}

impl<K, V> fmt::Debug for ErrorWithValue<K, V>
where
    K: ::Kind,
    V: fmt::Debug,
    <<K as ::Kind>::Check as ::Check>::Error: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "ErrorWithValue({:?}, {:?})", self.0, self.1)
    }
}

impl<K, V> error::Error for ErrorWithValue<K, V>
where
    K: ::Kind,
    V: fmt::Debug,
    <<K as ::Kind>::Check as ::Check>::Error: error::Error,
{
    fn description(&self) -> &str { "text check error with value" }

    fn cause(&self) -> Option<&error::Error> { Some(&self.0) }
}

impl<K, V> fmt::Display for ErrorWithValue<K, V>
where
    K: ::Kind,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "invalid {}", K::DESCRIPTION)
    }
}

impl<K, V> Eq for ErrorWithValue<K, V>
where
    K: ::Kind,
    <<K as ::Kind>::Check as ::Check>::Error: Eq,
    V: Eq,
{ }

impl<K, V> PartialEq for ErrorWithValue<K, V>
where
    K: ::Kind,
    <<K as ::Kind>::Check as ::Check>::Error: PartialEq,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

/// An error without an associated value.
///
/// This error is returned when the caller keeps the ownership of the validated value.
/// You can use [`with_value`](#method.with_value) to associate a value creating an
/// `ErrorWithValue`.
pub struct Error<K>(
    pub <<K as ::Kind>::Check as ::Check>::Error,
) where
    K: ::Kind;

impl<K> Error<K>
where
    K: ::Kind,
{
    /// Associate a value with this error and turn it into an `ErrorWithValue`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate textkind;
    ///
    /// let input = "invalid\ntitle".to_string();
    /// let error_without_value =
    ///     textkind::Title::<String>::try_from_str(&input)
    ///     .err()
    ///     .expect("input with control characters is not a valid title");
    ///
    /// let error_with_value = error_without_value
    ///     .with_value(input);
    ///
    /// assert_eq!(error_with_value.value(), "invalid\ntitle");
    /// ```
    pub fn with_value<V>(self, value: V) -> ErrorWithValue<K, V> {
        ErrorWithValue(self.0, value)
    }
}

impl<K> Clone for Error<K>
where
    K: ::Kind,
    <<K as ::Kind>::Check as ::Check>::Error: Clone,
{
    fn clone(&self) -> Self { Error(self.0.clone()) }
}

impl<K> fmt::Debug for Error<K>
where
    K: ::Kind,
    <<K as ::Kind>::Check as ::Check>::Error: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Error({:?})", self.0)
    }
}

impl<K> error::Error for Error<K>
where
    K: ::Kind,
    <<K as ::Kind>::Check as ::Check>::Error: error::Error,
{
    fn description(&self) -> &str { "text check error" }

    fn cause(&self) -> Option<&error::Error> { Some(&self.0) }
}

impl<K> fmt::Display for Error<K>
where
    K: ::Kind,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "invalid {}", K::DESCRIPTION)
    }
}

impl<K> Eq for Error<K>
where
    K: ::Kind,
    <<K as ::Kind>::Check as ::Check>::Error: Eq,
{ }

impl<K> PartialEq for Error<K>
where
    K: ::Kind,
    <<K as ::Kind>::Check as ::Check>::Error: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

