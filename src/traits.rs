
use std::borrow;
use std::rc;
use std::sync;

/// Value verification trait.
///
/// This trait must be implemented for a type for it to act as `Check` type for a `Kind`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// use std::fmt;
/// use std::error;
///
/// struct Sentence;
///
/// #[derive(Debug)]
/// enum SentenceError {
///     TooManyTerminators,
///     MissingTerminator,
/// }
///
/// impl fmt::Display for SentenceError {
///
///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
///         match *self {
///             SentenceError::TooManyTerminators =>
///                 write!(fmt, "too many sentence terminators"),
///             SentenceError::MissingTerminator =>
///                 write!(fmt, "missing sentence terminator"),
///         }
///     }
/// }
///
/// impl error::Error for SentenceError {
///
///     fn description(&self) -> &str { "sentence error" }
/// }
///
/// impl textkind::Check for Sentence {
///
///     type Error = SentenceError;
///
///     fn check(value: &str) -> Result<(), Self::Error> {
///         let is_terminator = |c| c == '.' || c == '?' || c == '!';
///         if value.chars().filter(|c| is_terminator(*c)).count() > 1 {
///             Err(SentenceError::TooManyTerminators)
///         } else if !value.ends_with(is_terminator) {
///             Err(SentenceError::MissingTerminator)
///         } else {
///             Ok(())
///         }
///     }
/// }
///
/// # (|| {
/// use textkind::Check;
/// assert!(Sentence::check("Is this a sentence?").is_ok());
/// assert!(Sentence::check("Is this? No.").is_err());
/// assert!(Sentence::check("Neither is this").is_err());
/// # })();
/// # Ok(())
/// # }
/// ```
pub trait Check {

    /// The error that will be returned when an invalid value is checked.
    type Error;

    /// Checks the given value for validity.
    ///
    /// # Errors
    ///
    /// Returns the specified `Self::Error` if the given value is invalid.
    fn check(value: &str) -> Result<(), Self::Error>;
}

/// Value identity trait.
///
/// Identifies a kind of text. This provides type safety for different text kinds, but also
/// declares the checks that need to be done to verify if a text is valid.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
///
/// struct Id;
///
/// impl textkind::Kind for Id {
///     type Check = textkind::check::IdentifierLax;
///     const DESCRIPTION: &'static str = "id";
/// }
///
/// let id: textkind::Text<Id, String> = textkind::Text::try_from_str("foo")?;
/// assert_eq!(id.as_str(), "foo");
/// # Ok(())
/// # }
/// ```
pub trait Kind {

    /// The check type determining if a given value is valid for this kind.
    type Check: Check;

    /// A simple description of this kind. This is used in error messages.
    const DESCRIPTION: &'static str;
}

/// Dynamic storage trait.
///
/// This trait is implemented for types that provide dynamic storage for text values.
pub trait Dynamic: Clone {

    /// Construct the dynamic storage from a `&str` slice.
    ///
    /// This will delegate to [`from_string`](#method.from_string) by default.
    ///
    /// A type should implement this method if it cannot take over a `String` as storage.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate textkind;
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// use std::sync::Arc;
    /// use textkind::Dynamic;
    ///
    /// let value: Arc<String> = Dynamic::from_str("foo");
    /// assert_eq!(value.as_str(), "foo");
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(value: &str) -> Self {
        Self::from_string(value.into())
    }

    /// Construct the dynamic storage from a `std::borrow::Cow<str>` value.
    ///
    /// This will delegate to [`from_string`](#method.from_string) by default.
    ///
    /// A type should implement this method if it cannot take over a `String` as storage.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate textkind;
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// use std::sync::Arc;
    /// use std::borrow::Cow;
    /// use textkind::Dynamic;
    ///
    /// let value: Arc<String> = Dynamic::from_cow(Cow::Borrowed("foo"));
    /// assert_eq!(value.as_str(), "foo");
    /// # Ok(())
    /// # }
    /// ```
    fn from_cow(value: borrow::Cow<str>) -> Self {
        Self::from_string(value.into())
    }

    /// Construct the dynamic storage from a `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate textkind;
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// use std::sync::Arc;
    /// use textkind::Dynamic;
    ///
    /// let value: Arc<String> = Dynamic::from_string("foo".to_string());
    /// assert_eq!(value.as_str(), "foo");
    /// # Ok(())
    /// # }
    /// ```
    fn from_string(value: String) -> Self;

    /// Construct the dynamic storage from another dynamic storage.
    ///
    /// The implementing type should use [`as_str`](#method.as_str),
    /// [`try_extract_string`](#method.try_extract_string) or
    /// [`into_string`](#method.into_string) to construct its new value as appropriate.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate textkind;
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// use std::sync::Arc;
    /// use textkind::Dynamic;
    ///
    /// let value: Arc<String> = Dynamic::from("foo".to_string());
    /// assert_eq!(value.as_str(), "foo");
    /// # Ok(())
    /// # }
    /// ```
    fn from<D>(dynamic: D) -> Self
    where
        D: Dynamic;

    /// Return a `&str` view into the stored value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate textkind;
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// use std::sync::Arc;
    /// use textkind::Dynamic;
    ///
    /// let value: Arc<String> = Dynamic::from_str("foo");
    /// assert_eq!(value.as_str(), "foo");
    /// # Ok(())
    /// # }
    /// ```
    fn as_str(&self) -> &str;

    /// Attempt to extract a `String` from the dynamic storage.
    ///
    /// This will signal extraction failure by default.
    ///
    /// A type should implement this method if it can potentially extract the `String` from
    /// the storage and return it for reuse.
    ///
    /// # Errors
    ///
    /// Returns `Self` as an `Err(_)` if a `String` could not be extracted.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate textkind;
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// use std::sync::Arc;
    /// use textkind::Dynamic;
    ///
    /// let value: Arc<String> = Dynamic::from_str("foo");
    /// match value.try_extract_string() {
    ///     Ok(value) => println!("extracted string {:?}", value),
    ///     Err(value) => println!("unable to extract from {:?}", value),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn try_extract_string(self) -> Result<String, Self> { Err(self) }

    /// Extract or construct a `String` from the value.
    ///
    /// This will try to extract a `String` via
    /// [`try_extract_string`](#method.try_extract_string) and if not possible construct a
    /// new `String` via [`as_str`](#method.as_str) by default.
    ///
    /// A type may implement this method instead of
    /// [`try_extract_string`](#method.try_extract_string) if there is no way it will be
    /// able to extract a stored `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate textkind;
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// use std::sync::Arc;
    /// use textkind::Dynamic;
    ///
    /// let value: Arc<String> = Dynamic::from_str("foo");
    /// let string = value.into_string();
    /// assert_eq!(&string, "foo");
    /// # Ok(())
    /// # }
    /// ```
    fn into_string(self) -> String {
        match self.try_extract_string() {
            Ok(value) => value,
            Err(dynamic) => dynamic.as_str().into(),
        }
    }
}

/// Implementation of `Dynamic` for `String`.
impl Dynamic for String {

    /// Use the passed `String` as dynamic storage.
    fn from_string(value: String) -> Self { value }

    /// Use the dynamic storage's `into_string` method to construct a `String` and uses
    /// that as dynamic storage.
    fn from<D>(dynamic: D) -> Self
    where
        D: Dynamic,
    {
        dynamic.into_string()
    }

    /// Fetch the `&str` slice from the `String`.
    fn as_str(&self) -> &str { self }

    /// Return the dynamic storage.
    fn into_string(self) -> String { self }

    /// Return the dynamic storage.
    fn try_extract_string(self) -> Result<String, Self> { Ok(self) }
}

/// Implementation of `Dynamic` for reference counted `String`s.
impl Dynamic for rc::Rc<String> {

    /// Wrap the `String` in a `std::rc::Rc<_>`.
    fn from_string(value: String) -> Self { rc::Rc::new(value) }

    /// Wrap the other storage's `into_string` result in a `std::rc::Rc<_>`.
    fn from<D>(dynamic: D) -> Self
    where
        D: Dynamic,
    {
        rc::Rc::new(dynamic.into_string())
    }

    /// Fetch the `&str` slice from the `std::rc::Rc<String>`.
    fn as_str(&self) -> &str { self }

    /// Extract the `String` from the `std::rc::Rc<String>` if the current handle
    /// to the shared storage is the only one.
    fn try_extract_string(self) -> Result<String, Self> {
        rc::Rc::try_unwrap(self)
    }
}

/// Implementation of `Dynamic` for atomically reference counted `String`s.
impl Dynamic for sync::Arc<String> {

    /// Wrap the `String` in a `std::sync::Arc<_>`.
    fn from_string(value: String) -> Self { sync::Arc::new(value) }

    /// Wrap the other storage's `into_string` result in a `std::sync::Arc<_>`.
    fn from<D>(dynamic: D) -> Self
    where
        D: Dynamic,
    {
        sync::Arc::new(dynamic.into_string())
    }

    /// Fetch the `&str` slice from the `std::sync::Arc<String>`.
    fn as_str(&self) -> &str { self }

    /// Extract the `String` from the `std::sync::Arc<String>` if the current handle
    /// to the shared storage is the only one.
    fn try_extract_string(self) -> Result<String, Self> {
        sync::Arc::try_unwrap(self)
    }
}

