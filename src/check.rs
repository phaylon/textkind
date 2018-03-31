//! Predefined check types.
//!
//! This is a collection of common value checks.
//!
//! See the `kind` module for a collection of predefined text kinds.
//!
//! See the `Kind` trait for an example on how to associate a check with a kind.

use std::error;
use std::fmt;

/// Non-empty text without control characters or leading/trailing whitespace.
pub type Title = And<NotEmpty, And<NoControl, Trimmed>>;

/// Signals that a value is invalid because it is empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotEmptyError;

impl error::Error for NotEmptyError {

    fn description(&self) -> &str { "NotEmpty error" }
}

impl fmt::Display for NotEmptyError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "value is empty")
    }
}


/// Ensure a value is not empty.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// use textkind::Check;
///
/// assert!(textkind::check::NotEmpty::check("foo").is_ok());
/// assert!(textkind::check::NotEmpty::check("").is_err());
/// # Ok(())
/// # }
/// ```
#[allow(missing_debug_implementations)]
pub struct NotEmpty {
    _unconstructable: ::Void,
}

impl ::Check for NotEmpty {

    type Error = NotEmptyError;

    fn check(value: &str) -> Result<(), Self::Error> {
        if value.is_empty() {
            Err(NotEmptyError)
        } else {
            Ok(())
        }
    }
}

/// Signals that a value is invalid because it contained a newline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleLineError;

impl error::Error for SingleLineError {

    fn description(&self) -> &str { "SingleLine error" }
}

impl fmt::Display for SingleLineError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "value is not single-line")
    }
}

/// Ensure a value does not contain newlines and is therefor on a single line.
///
/// A trailing newline will also cause the check to fail.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// use textkind::Check;
///
/// assert!(textkind::check::SingleLine::check("foo").is_ok());
/// assert!(textkind::check::SingleLine::check("").is_ok());
///
/// assert!(textkind::check::SingleLine::check("foo\nbar").is_err());
/// assert!(textkind::check::SingleLine::check("foo\n").is_err());
/// assert!(textkind::check::SingleLine::check("\n").is_err());
/// # Ok(())
/// # }
/// ```
#[allow(missing_debug_implementations)]
pub struct SingleLine {
    _unconstructable: ::Void,
}

impl ::Check for SingleLine {

    type Error = SingleLineError;
    
    fn check(value: &str) -> Result<(), Self::Error> {
        if value.find('\n').is_none() {
            Ok(())
        } else {
            Err(SingleLineError)
        }
    }
}

/// Signals that a value is invalid because it contained whitespaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoWhitespaceError {
    /// The number of whitespace characters that were found.
    pub whitespace_count: usize,
}

impl error::Error for NoWhitespaceError {

    fn description(&self) -> &str { "NoWhitespace error" }
}

impl fmt::Display for NoWhitespaceError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "value contains whitespace (value contains {} whitespace sequence(s))",
            self.whitespace_count,
        )
    }
}

/// Ensure a value does not contain whitespaces.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// use textkind::Check;
///
/// assert!(textkind::check::NoWhitespace::check("foo").is_ok());
/// assert!(textkind::check::NoWhitespace::check("").is_ok());
///
/// assert!(textkind::check::NoWhitespace::check("foo bar").is_err());
/// assert!(textkind::check::NoWhitespace::check("foo\nbar").is_err());
/// assert!(textkind::check::NoWhitespace::check("foo\tbar").is_err());
/// assert!(textkind::check::NoWhitespace::check("\t").is_err());
/// # Ok(())
/// # }
/// ```
#[allow(missing_debug_implementations)]
pub struct NoWhitespace {
    _unconstructable: ::Void,
}

impl ::Check for NoWhitespace {

    type Error = NoWhitespaceError;
    
    fn check(value: &str) -> Result<(), Self::Error> {
        let whitespace_count = value.split(|c: char| c.is_whitespace()).count() - 1;
        if whitespace_count == 0 {
            Ok(())
        } else {
            Err(NoWhitespaceError { whitespace_count })
        }
    }
}

/// Signals that a value is invalid because it contained control characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoControlError {
    /// The number of control characters that were found.
    pub control_count: usize,
}

impl error::Error for NoControlError {

    fn description(&self) -> &str { "NoControl error" }
}

impl fmt::Display for NoControlError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "value contains {} control character(s)",
            self.control_count,
        )
    }
}

/// Ensure a value does not contain control characters.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// use textkind::Check;
///
/// assert!(textkind::check::NoControl::check("foo").is_ok());
/// assert!(textkind::check::NoControl::check("").is_ok());
/// assert!(textkind::check::NoControl::check("foo bar").is_ok());
///
/// assert!(textkind::check::NoControl::check("foo\nbar").is_err());
/// assert!(textkind::check::NoControl::check("foo\tbar").is_err());
/// assert!(textkind::check::NoControl::check("\t").is_err());
/// # Ok(())
/// # }
/// ```
#[allow(missing_debug_implementations)]
pub struct NoControl {
    _unconstructable: ::Void,
}

impl ::Check for NoControl {

    type Error = NoControlError;
    
    fn check(value: &str) -> Result<(), Self::Error> {
        let control_count = value.chars().filter(|c| c.is_control()).count();
        if control_count == 0 {
            Ok(())
        } else {
            Err(NoControlError { control_count })
        }
    }
}

/// Signals that a value is invalid because it failed a check when trimmed.
///
/// The contained value is the error of the failed check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhenTrimmedError<T>(pub T);

impl<E> error::Error for WhenTrimmedError<E>
where
    E: error::Error,
{
    fn description(&self) -> &str { "Trimmed error" }
}

impl<E> fmt::Display for WhenTrimmedError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} when trimmed", self.0)
    }
}

/// Ensure a value passes a check when whitespace is trimmed off the beginning and end.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// use textkind::Check;
///
/// type NotEmpty =
///     textkind::check::NotEmpty;
/// type NotEmptyTrimmed =
///     textkind::check::WhenTrimmed<textkind::check::NotEmpty>;
///
/// assert!(NotEmpty::check("").is_err());
/// assert!(NotEmpty::check("  ").is_ok());
///
/// assert!(NotEmptyTrimmed::check("").is_err());
/// assert!(NotEmptyTrimmed::check("  ").is_err());
/// # Ok(())
/// # }
/// ```
#[allow(missing_debug_implementations)]
pub struct WhenTrimmed<T> {
    _inner: T,
    _unconstructable: ::Void,
}

impl<T> ::Check for WhenTrimmed<T>
where
    T: ::Check,
{
    type Error = WhenTrimmedError<T::Error>;

    fn check(value: &str) -> Result<(), Self::Error> {
        T::check(value.trim()).map_err(WhenTrimmedError)
    }
}

/// Signals that a value is invalid because it failed one of two checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndError<E1, E2> {
    /// The left check has failed with the enclosed error.
    Err1(E1),
    /// The right check has failed with the enclosed error.
    Err2(E2),
}

impl<E1, E2> error::Error for AndError<E1, E2>
where
    E1: error::Error,
    E2: error::Error,
{
    fn description(&self) -> &str { "combined And error" }
}

impl<E1, E2> fmt::Display for AndError<E1, E2>
where
    E1: fmt::Display,
    E2: fmt::Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AndError::Err1(ref error) => fmt::Display::fmt(error, fmt),
            AndError::Err2(ref error) => fmt::Display::fmt(error, fmt),
        }
    }
}

/// Ensure a value passes two checks.
///
/// This type can be nested to combine any number of checks.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// use textkind::Check;
///
/// type NotEmptyNoControl = textkind::check::And<
///     textkind::check::NotEmpty,
///     textkind::check::NoControl,
/// >;
///
/// assert!(NotEmptyNoControl::check("").is_err());
/// assert!(NotEmptyNoControl::check("\t").is_err());
///
/// # Ok(())
/// # }
/// ```
#[allow(missing_debug_implementations)]
pub struct And<T1, T2> {
    _check_1: T1,
    _check_2: T2,
    _unconstructable: ::Void,
}

impl<T1, T2> ::Check for And<T1, T2>
where
    T1: ::Check,
    T2: ::Check,
{
    type Error = AndError<T1::Error, T2::Error>;

    fn check(value: &str) -> Result<(), Self::Error> {
        T1::check(value)
            .map_err(AndError::Err1)
            .and_then(|()| T2::check(value).map_err(AndError::Err2))
    }
}

/// Signals that a value is invalid because it begins with whitespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrimmedLeftError;

impl error::Error for TrimmedLeftError {

    fn description(&self) -> &str { "TrimmedLeft error" }
}

impl fmt::Display for TrimmedLeftError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "value has whitespace at the end")
    }
}

/// Ensure a value doesn't start with whitespace.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// use textkind::Check;
///
/// assert!(textkind::check::TrimmedLeft::check("foo  ").is_ok());
/// assert!(textkind::check::TrimmedLeft::check("  foo").is_err());
///
/// # Ok(())
/// # }
/// ```
#[allow(missing_debug_implementations)]
pub struct TrimmedLeft {
    _unconstructable: ::Void,
}

impl ::Check for TrimmedLeft {

    type Error = TrimmedLeftError;

    fn check(value: &str) -> Result<(), Self::Error> {
        if value.len() == value.trim_left().len() {
            Ok(())
        } else {
            Err(TrimmedLeftError)
        }
    }
}

/// Signals that a value is invalid because it ends with whitespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrimmedRightError;

impl error::Error for TrimmedRightError {

    fn description(&self) -> &str { "TrimmedRight error" }
}

impl fmt::Display for TrimmedRightError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "value has whitespace at the beginning")
    }
}

/// Ensure a value doesn't end with whitespace.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// use textkind::Check;
///
/// assert!(textkind::check::TrimmedRight::check("  foo").is_ok());
/// assert!(textkind::check::TrimmedRight::check("foo  ").is_err());
///
/// # Ok(())
/// # }
/// ```
#[allow(missing_debug_implementations)]
pub struct TrimmedRight {
    _unconstructable: ::Void,
}

impl ::Check for TrimmedRight {

    type Error = TrimmedRightError;

    fn check(value: &str) -> Result<(), Self::Error> {
        if value.len() == value.trim_right().len() {
            Ok(())
        } else {
            Err(TrimmedRightError)
        }
    }
}

/// Signals that a value is invalid because it only contains whitespace.
///
/// This is used for improved error messages when a value must be trimmed but only contains
/// whitespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrimmedOnlyError;

impl error::Error for TrimmedOnlyError {

    fn description(&self) -> &str { "TrimmedOnly error" }
}

impl fmt::Display for TrimmedOnlyError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "value contains only whitespace characters")
    }
}

/// Signals that a value is invalid because it starts and ends with whitespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrimmedBothError;

impl error::Error for TrimmedBothError {

    fn description(&self) -> &str { "TrimmedBoth error" }
}

impl fmt::Display for TrimmedBothError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "value has whitespace at beginning and end")
    }
}

/// Signals that a value is invalid because it starts or ends with whitespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrimmedError {
    /// The value is invalid because it starts with whitespace.
    Left(TrimmedLeftError),
    /// The value is invalid because it ends with whitespace.
    Right(TrimmedRightError),
    /// The value is invalid because it starts and ends with whitespace.
    Both(TrimmedBothError),
    /// The value is invalid because it only contains whitespace.
    Only(TrimmedOnlyError),
}

impl error::Error for TrimmedError {

    fn description(&self) -> &str { "Trimmed error" }
}

impl fmt::Display for TrimmedError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TrimmedError::Left(ref error) => fmt::Display::fmt(error, fmt),
            TrimmedError::Right(ref error) => fmt::Display::fmt(error, fmt),
            TrimmedError::Both(ref error) => fmt::Display::fmt(error, fmt),
            TrimmedError::Only(ref error) => fmt::Display::fmt(error, fmt),
        }
    }
}

/// Ensure a value doesn't begin or end with whitespace.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// use textkind::Check;
///
/// assert!(textkind::check::Trimmed::check("foo").is_ok());
/// assert!(textkind::check::Trimmed::check("").is_ok());
///
/// assert!(textkind::check::Trimmed::check("foo  ").is_err());
/// assert!(textkind::check::Trimmed::check("  foo").is_err());
/// assert!(textkind::check::Trimmed::check(" foo ").is_err());
/// assert!(textkind::check::Trimmed::check("  ").is_err());
/// # Ok(())
/// # }
/// ```
#[allow(missing_debug_implementations)]
pub struct Trimmed {
    _unconstructable: ::Void,
}

impl ::Check for Trimmed {

    type Error = TrimmedError;

    fn check(value: &str) -> Result<(), Self::Error> {
        if !value.is_empty() && value.trim().is_empty() {
            Err(TrimmedError::Only(TrimmedOnlyError))
        } else {
            match (TrimmedLeft::check(value), TrimmedRight::check(value)) {
                (Ok(()), Ok(())) => Ok(()),
                (Err(error), Ok(())) => Err(TrimmedError::Left(error)),
                (Ok(()), Err(error)) => Err(TrimmedError::Right(error)),
                (Err(_), Err(_)) => Err(TrimmedError::Both(TrimmedBothError)),
            }
        }
    }
}

/// Signals that a value is not a valid lax identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifierLaxError {
    /// The value is empty.
    Empty(NotEmptyError),
    /// The value contains an invalid character.
    InvalidChar(char),
}

impl error::Error for IdentifierLaxError {

    fn description(&self) -> &str { "IdentifierLax error" }
}

impl fmt::Display for IdentifierLaxError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IdentifierLaxError::Empty(ref error) =>
                fmt::Display::fmt(error, fmt),
            IdentifierLaxError::InvalidChar(c) =>
                write!(fmt, "value contains invalid character `{}`", c.escape_default()),
        }
    }
}

/// Ensure a value is a valid relaxed identifier.
///
/// To be a valid relaxed identifier, a value has to be not empty and only contain the
/// following characters:
///
/// * `A` to `Z` (uppercase ASCII alphabetic characters)
/// * `a` to `z` (lowercase ASCII alphabetic characters)
/// * `0` to `9` (ASCII digits)
/// * `_` (underscore)
/// * `-` (hyphen)
///
/// These characters can appear in any position in the value.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// use textkind::Check;
///
/// assert!(textkind::check::IdentifierLax::check("foo").is_ok());
/// assert!(textkind::check::IdentifierLax::check("foo-bar").is_ok());
/// assert!(textkind::check::IdentifierLax::check("23").is_ok());
///
/// assert!(textkind::check::IdentifierLax::check("foo bar").is_err());
/// assert!(textkind::check::IdentifierLax::check("").is_err());
/// # Ok(())
/// # }
/// ```
#[allow(missing_debug_implementations)]
pub struct IdentifierLax {
    _unconstructable: ::Void,
}

impl ::Check for IdentifierLax {

    type Error = IdentifierLaxError;

    fn check(value: &str) -> Result<(), Self::Error> {
        NotEmpty::check(value).map_err(IdentifierLaxError::Empty)?;
        for c in value.chars() {
            match c {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' | '-' => (),
                _ => return Err(IdentifierLaxError::InvalidChar(c)),
            }
        }
        Ok(())
    }
}

/// Signals that a value is not a valid lax identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifierError {
    /// The value is empty.
    Empty(NotEmptyError),
    /// The value begins with an invalid character.
    InvalidStartChar(char),
    /// One of the characters after the first is invalid.
    InvalidRestChar(char),
}

impl error::Error for IdentifierError {

    fn description(&self) -> &str { "Identifier error" }
}

impl fmt::Display for IdentifierError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IdentifierError::Empty(ref error) =>
                fmt::Display::fmt(error, fmt),
            IdentifierError::InvalidStartChar(c) =>
                write!(fmt, "value begins with invalid character `{}`", c.escape_default()),
            IdentifierError::InvalidRestChar(c) =>
                write!(fmt, "value contains invalid character `{}`", c.escape_default()),
        }
    }
}

/// Ensure a value is a valid identifier.
///
/// To be a valid identifier, a value has to be not empty and only contain the following
/// characters:
///
/// * `A` to `Z` (uppercase ASCII alphabetic characters)
/// * `a` to `z` (lowercase ASCII alphabetic characters)
/// * `0` to `9` (ASCII digits, **not allowed at the beginning**)
/// * `_` (underscore)
///
/// All but the ASCII digit characters can appear in any position in the value.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// extern crate textkind;
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// use textkind::Check;
///
/// assert!(textkind::check::Identifier::check("foo").is_ok());
/// assert!(textkind::check::Identifier::check("foo_bar").is_ok());
/// assert!(textkind::check::Identifier::check("foo23").is_ok());
///
/// assert!(textkind::check::Identifier::check("foo-bar").is_err());
/// assert!(textkind::check::Identifier::check("23").is_err());
/// assert!(textkind::check::Identifier::check("foo bar").is_err());
/// assert!(textkind::check::Identifier::check("").is_err());
/// # Ok(())
/// # }
/// ```
#[allow(missing_debug_implementations)]
pub struct Identifier {
    _unconstructable: ::Void,
}

impl ::Check for Identifier {

    type Error = IdentifierError;

    fn check(value: &str) -> Result<(), Self::Error> {
        NotEmpty::check(value).map_err(IdentifierError::Empty)?;
        let mut chars = value.chars();
        let start_char = chars.next().expect("non-empty value has at least one char");
        match start_char {
            'a'...'z' | 'A'...'Z' | '_' => (),
            _ => return Err(IdentifierError::InvalidStartChar(start_char)),
        }
        for rest_char in chars {
            match rest_char {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => (),
                _ => return Err(IdentifierError::InvalidRestChar(rest_char)),
            }
        }
        Ok(())
    }
}

/// Signals that a value is too large bytewise to be valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxBytesError {
    /// Maximum allowed byte length.
    pub max: usize,
    /// Actual byte length of the value.
    pub len: usize,
}

impl error::Error for MaxBytesError {

    fn description(&self) -> &str { "MaxBytes error" }
}

impl fmt::Display for MaxBytesError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "length of {} exceeds limit of {}", self.len, self.max)
    }
}

macro_rules! gen_max_bytes {
    ($name:ident: $max:expr) => {

        /// Ensure a value has a byte count lower than the specified number.
        ///
        /// # Examples
        ///
        /// Basic usage for `MaxBytes256`. The other `MaxBytes*` checks work the same but
        /// check for different byte lengths.
        ///
        /// ```
        /// extern crate textkind;
        /// # fn main() { example().expect("no errors") }
        /// # fn example() -> Result<(), Box<::std::error::Error>> {
        /// use textkind::Check;
        ///
        /// let valid = "X".repeat(256);
        /// let invalid = "X".repeat(257);
        ///
        /// assert!(textkind::check::MaxBytes256::check(&valid).is_ok());
        /// assert!(textkind::check::MaxBytes256::check(&invalid).is_err());
        /// # Ok(())
        /// # }
        /// ```
        #[allow(missing_debug_implementations)]
        pub struct $name {
            _unconstructable: ::Void,
        }

        impl ::Check for $name {

            type Error = MaxBytesError;

            fn check(value: &str) -> Result<(), Self::Error> {
                if value.as_bytes().len() <= $max {
                    Ok(())
                } else {
                    Err(MaxBytesError {
                        max: $max,
                        len: value.as_bytes().len(),
                    })
                }
            }
        }
    }
}

gen_max_bytes!(MaxBytes256: 256);
gen_max_bytes!(MaxBytes512: 512);
gen_max_bytes!(MaxBytes1024: 1024);

