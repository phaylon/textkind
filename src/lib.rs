#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
)]
#![deny(
    unsafe_code,
)]
//! Typed text kinds.
//!
//! This crate provides a `Text<Kind, DynamicStorage>` wrapper for text values providing the
//! following features:
//!
//! * Different kinds of texts have different types.
//! * Value validation associated with text kinds.
//! * Special storage for `&'static str` values avoiding allocation.
//! * Can sometimes avoid allocation for small strings.
//! * Parameterised dynamic storage (`String`, `Rc<String>` or `Arc<String>`).
//! * Checked conversions between kinds.
//! * Transition from one dynamic storage to another.
//! * Optional [serde](https://crates.io/crates/serde) integration.
//!
//! The code is not performance-oriented and kept rather simple. The dynamic storage parameter
//! merely allows avoiding unnecessary copies. The API is also focused on text values that
//! don't change a lot.
//!
//! # Features
//!
//! * `serde` adds [serde](https://crates.io/crates/serde) serialization and deserialization.
//!
//! # Examples
//!
//! Basic usage of predefined types:
//!
//! ```
//! extern crate textkind;
//! # fn main() { example().expect("no errors") }
//! # fn example() -> Result<(), Box<::std::error::Error>> {
//!
//! let title = textkind::Title::<String>::try_from_str("Some Title")?;
//! println!("Title: {}", title);
//! # Ok(())
//! # }
//! ```
//!
//! Custom kind implementation:
//!
//! ```
//! extern crate textkind;
//! # fn main() { example().expect("no errors") }
//! # fn example() -> Result<(), Box<::std::error::Error>> {
//!
//! struct SummaryKind;
//!
//! impl textkind::Kind for SummaryKind {
//!     type Check = textkind::check::NotEmpty;
//!     const DESCRIPTION: &'static str = "summary";
//! }
//!
//! type Summary<D> = textkind::Text<SummaryKind, D>;
//!
//! let summary = Summary::<String>::try_from_str("Some Summry")?;
//! println!("Summary: {}", summary);
//! # Ok(())
//! # }
//! ```
//!
//! Custom check implementation:
//!
//! ```
//! extern crate textkind;
//! # fn main() { example().expect("no errors") }
//! # fn example() -> Result<(), Box<::std::error::Error>> {
//!
//! struct UppercaseCheck;
//!
//! #[derive(Debug)]
//! struct UppercaseError;
//!
//! impl textkind::Check for UppercaseCheck {
//!     type Error = UppercaseError;
//!     fn check(value: &str) -> Result<(), Self::Error> {
//!         if value.chars().all(|c| c.is_uppercase()) {
//!             Ok(())
//!         } else {
//!             Err(UppercaseError)
//!         }
//!     }
//! }
//!
//! impl ::std::fmt::Display for UppercaseError {
//!     fn fmt(
//!         &self,
//!         fmt: &mut ::std::fmt::Formatter,
//!     ) -> ::std::fmt::Result {
//!         write!(fmt, "value is not all uppercase")
//!     }
//! }
//!
//! impl ::std::error::Error for UppercaseError {
//!     fn description(&self) -> &str { "uppercase error" }
//! }
//!
//! struct UppercaseKind;
//!
//! impl textkind::Kind for UppercaseKind {
//!     type Check = UppercaseCheck;
//!     const DESCRIPTION: &'static str = "uppercase text";
//! }
//!
//! type Uppercase<D> = textkind::Text<UppercaseKind, D>;
//!
//! assert!(Uppercase::<String>::try_from_str("FOO").is_ok());
//! assert!(Uppercase::<String>::try_from_str("foo").is_err());
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "serde")]
extern crate serde;

use std::borrow;
use std::cmp;
use std::fmt;
use std::hash;
use std::marker;
use std::ops;
use std::str;

pub mod check;
pub mod kind;

mod conversion;
pub use conversion::*;

mod data;
pub use data::*;

mod errors;
pub use errors::*;

mod small;
pub use small::*;

mod traits;
pub use traits::*;

#[cfg(feature = "serde")]
mod serde_support;

/// Predefined title text type.
///
/// This uses `kind::Title` as a text kind while keeping the dynamic storage as a type parameter.
pub type Title<D> = Text<kind::Title, D>;

/// Predefined identifier text type.
///
/// This uses `kind::Identifier` as a text kind while keeping the dynamic storage as a type 
/// parameter.
pub type Identifier<D> = Text<kind::Identifier, D>;

/// Predefined lax identifier text type.
///
/// This uses `kind::IdentifierLax` as a text kind while keeping the dynamic storage as a type
/// parameter.
pub type IdentifierLax<D> = Text<kind::IdentifierLax, D>;

// Used to make kind and check types unconstructable.
enum Void {}

macro_rules! error_with_value {
    ($value:ident, $result:expr) => {{
        match $result {
            Ok(()) => Ok($value),
            Err(error) => Err(ErrorWithValue(error, $value)),
        }
    }}
}

/// Encapsulates a modification result.
///
/// This is used to indicate if a modified value is a new value or a subslice of an
/// existing one. The subslice variant allows static text values to avoid an allocation
/// unless required.
///
/// The type parameter `T` will usually be a dynamic storage type as used for `Text`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modified<'a, T> {
    /// Modification result is a completely new value.
    New(T),
    /// Modification result is a subslice of the existing value.
    Sub(&'a str),
}

impl<'a, D> From<D> for Modified<'a, D>
where
    D: Dynamic,
{
    fn from(value: D) -> Modified<'a, D> { Modified::New(value) }
}

impl<'a, D> From<&'a str> for Modified<'a, D>
where
    D: Dynamic,
{
    fn from(value: &'a str) -> Modified<'a, D> { Modified::Sub(value) }
}

/// Owned text value with parameterisable identity and dynamic storage.
///
/// This is the main type of this crate. It requires two type parameters:
///
/// * `K` is a type implementing `Kind`. This represents the kind identity of the text. Every
///   text kind has an associated `Check` type to determine if a given value is valid.
/// * `D` is a type providing `Dynamic` storage for the text. This is something like `String`,
///   `Arc<String>` or `Rc<String>`.
///
/// Special constructors for `&'static str` values are available that allow avoiding dynamic
/// storage where possible.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # fn main() { example().expect("no errors") }
/// # fn example() -> Result<(), Box<::std::error::Error>> {
/// extern crate textkind;
///
/// struct MyText;
///
/// impl textkind::Kind for MyText {
///     type Check = textkind::check::NotEmpty;
///     const DESCRIPTION: &'static str = "my text";
/// }
///
/// let text: textkind::Text<MyText, String> =
///     textkind::Text::try_from_str("foo")?;
///
/// println!("the value is {}", text);
/// # Ok(())
/// # }
/// ```
pub struct Text<K, D> {
    _kind: marker::PhantomData<K>,
    data: Data<D>,
}

impl<K, D> Text<K, D>
where
    K: Kind,
    D: Dynamic,
{
    /// Attempt to construct this text value from a `&'static str`.
    ///
    /// This will directly store the static reference and avoid a possible allocation by the
    /// dynamic storage type.
    ///
    /// # Errors
    ///
    /// Returns an `Error<K>` without the associated value when the value is invalid.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// let text: textkind::Title<String> =
    ///     textkind::Title::try_from_static_str("foo")?;
    ///
    /// println!("the value is {}", text);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_from_static_str(value: &'static str) -> Result<Self, Error<K>> {
        K::Check::check(value).map_err(Error)?;
        Ok(Text {
            _kind: marker::PhantomData,
            data: Data::from_static_str(value),
        })
    }

    /// Attempt to construct this text value from a `&'_ str`.
    ///
    /// This will initialise a new dynamic storage with the given value. This will usually
    /// involve an allocation by the dynamic storage.
    ///
    /// # Errors
    ///
    /// Returns an `Error<K>` without the associated value when the value is invalid.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// let input = "foo".to_string();
    /// let text: textkind::Title<String> =
    ///     textkind::Title::try_from_str(&input)?;
    ///
    /// println!("the value is {}", text);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_from_str(value: &str) -> Result<Self, Error<K>> {
        K::Check::check(value).map_err(Error)?;
        Ok(Text {
            _kind: marker::PhantomData,
            data: Data::from_str(value),
        })
    }

    /// Attempt to construct this text value from a `std::borrow::Cow<'_ str>`.
    ///
    /// This method mainly exists because you sometimes already have a `std::borrow::Cow`
    /// wrapped value and want to defer the decision of reuse to the dynamic storage.
    ///
    /// # Errors
    ///
    /// Returns an `ErrorWithValue<K>` with the associated value when the value is invalid.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// let input = "foo".to_string();
    /// let text: textkind::Title<String> =
    ///     textkind::Title::try_from_str_cow(input.into())?;
    ///
    /// println!("the value is {}", text);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_from_str_cow(
        value: borrow::Cow<str>,
    ) -> Result<Self, ErrorWithValue<K, borrow::Cow<str>>> {
        let value = error_with_value!(value, K::Check::check(&value))?;
        Ok(Text {
            _kind: marker::PhantomData,
            data: Data::from_cow(value),
        })
    }

    /// Attempt to construct this text value from a `std::borrow::Cow<'static str>`.
    ///
    /// This is exactly like [`try_from_string`](#method.try_from_string) except it will not
    /// use the dynamic storage when the value is a `&'static str`. It means the caller doesn't
    /// potentially have to choose between [`try_from_string`](#method.try_from_string) and
    /// [`try_from_static_str`](#method.try_from_static_str).
    ///
    /// # Errors
    ///
    /// Returns an `ErrorWithValue<K>` with the associated value when the value is invalid.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// let text: textkind::Title<String> =
    ///     textkind::Title::try_from_static_str_cow("foo".into())?;
    ///
    /// println!("the value is {}", text);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_from_static_str_cow(
        value: borrow::Cow<'static, str>,
    ) -> Result<Self, ErrorWithValue<K, borrow::Cow<'static, str>>> {
        let value = error_with_value!(value, K::Check::check(&value))?;
        Ok(Text {
            _kind: marker::PhantomData,
            data: Data::from_static_str_cow(value),
        })
    }

    /// Attempt to construct this text value from a `String`.
    ///
    /// This constructor allows the dynamic storage to potentially take over ownership of the
    /// string and keep it instead of making a new allocation.
    ///
    /// # Errors
    ///
    /// Returns an `ErrorWithValue<K>` with the associated value when the value is invalid.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// let input = "foo".to_string();
    /// let text: textkind::Title<String> =
    ///     textkind::Title::try_from_string(input)?;
    ///
    /// println!("the value is {}", text);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_from_string(value: String) -> Result<Self, ErrorWithValue<K, String>> {
        let value = error_with_value!(value, K::Check::check(&value))?;
        Ok(Text {
            _kind: marker::PhantomData,
            data: Data::from_string(value),
        })
    }

    /// Attempt to construct this text value from an existing dynamic storage value.
    ///
    /// # Errors
    ///
    /// Returns an `ErrorWithValue<K>` with the associated storage when the value is invalid.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// let input = "foo".to_string();
    /// let text = textkind::Title::try_from_dynamic(input)?;
    /// println!("the value is {}", text);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_from_dynamic(value: D) -> Result<Self, ErrorWithValue<K, D>> {
        let value = error_with_value!(value, K::Check::check(value.as_str()))?;
        Ok(Text {
            _kind: marker::PhantomData,
            data: Data::from_dynamic(D::from(value)),
        })
    }

    /// Attempt to construct this text value from an existing data value.
    ///
    /// # Errors
    ///
    /// Returns an `ErrorWithValue<K>` with the associated data when the value is invalid.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// let input = "foo".to_string();
    /// let text = textkind::Title::try_from_data(
    ///     textkind::Data::Dynamic(input),
    /// )?;
    /// println!("the value is {}", text);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_from_data(data: Data<D>) -> Result<Self, ErrorWithValue<K, Data<D>>> {
        let data = error_with_value!(data, K::Check::check(data.as_str()))?;
        Ok(Text {
            _kind: marker::PhantomData,
            data,
        })
    }

    /// Convert from another kind via the `ConvertFrom` trait.
    ///
    /// # Panics
    ///
    /// Since this usually constructs a new text kind from an existing one, a call to this
    /// may run assertions that may panic.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// struct SourceKind;
    /// struct TargetKind;
    ///
    /// impl textkind::Kind for SourceKind {
    ///     type Check = textkind::check::Identifier;
    ///     const DESCRIPTION: &'static str = "source";
    /// }
    ///
    /// impl textkind::Kind for TargetKind {
    ///     type Check = textkind::check::Title;
    ///     const DESCRIPTION: &'static str = "target";
    /// }
    ///
    /// impl textkind::ConvertFrom<SourceKind> for TargetKind {
    ///
    ///     fn convert_from<D>(
    ///         source: textkind::Text<SourceKind, D>,
    ///     ) -> textkind::Text<TargetKind, D>
    ///     where
    ///         D: textkind::Dynamic,
    ///     {
    ///         textkind::Text::try_from_dynamic(source.into_dynamic())
    ///             .map_err(|error| error.without_value())
    ///             .expect("identifiers are always valid titles")
    ///     }
    /// }
    ///
    /// let source: textkind::Text<SourceKind, String> =
    ///     textkind::Text::try_from_string("foo".to_string())?;
    ///
    /// let target: textkind::Text<TargetKind, _> =
    ///     textkind::Text::convert_from(source);
    ///
    /// println!("target value is {}", target);
    /// # Ok(())
    /// # }
    /// ```
    pub fn convert_from<K2>(other: Text<K2, D>) -> Self
    where
        K2: Kind,
        K: ConvertFrom<K2>,
    {
        K::convert_from(other)
    }

    /// Convert to another kind via the `ConvertFrom` trait.
    ///
    /// # Panics
    ///
    /// Since this usually constructs a new text kind from an existing one, a call to this
    /// may run assertions that may panic.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// struct SourceKind;
    /// struct TargetKind;
    ///
    /// impl textkind::Kind for SourceKind {
    ///     type Check = textkind::check::Identifier;
    ///     const DESCRIPTION: &'static str = "source";
    /// }
    ///
    /// impl textkind::Kind for TargetKind {
    ///     type Check = textkind::check::Title;
    ///     const DESCRIPTION: &'static str = "target";
    /// }
    ///
    /// impl textkind::ConvertFrom<SourceKind> for TargetKind {
    ///
    ///     fn convert_from<D>(
    ///         source: textkind::Text<SourceKind, D>,
    ///     ) -> textkind::Text<TargetKind, D>
    ///     where
    ///         D: textkind::Dynamic,
    ///     {
    ///         textkind::Text::try_from_dynamic(source.into_dynamic())
    ///             .map_err(|error| error.without_value())
    ///             .expect("identifiers are always valid titles")
    ///     }
    /// }
    ///
    /// let source: textkind::Text<SourceKind, String> =
    ///     textkind::Text::try_from_string("foo".to_string())?;
    ///
    /// let target: textkind::Text<TargetKind, _> = source.convert_into();
    ///
    /// println!("target value is {}", target);
    /// # Ok(())
    /// # }
    /// ```
    pub fn convert_into<K2>(self) -> Text<K2, D>
    where
        K2: Kind,
        K2: ConvertFrom<K>,
    {
        K2::convert_from(self)
    }

    /// Try to convert from another text kind via the `TryConvertFrom` trait.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// struct SourceKind;
    /// struct TargetKind;
    ///
    /// impl textkind::Kind for SourceKind {
    ///     type Check = textkind::check::Identifier;
    ///     const DESCRIPTION: &'static str = "source";
    /// }
    ///
    /// impl textkind::Kind for TargetKind {
    ///     type Check = textkind::check::Title;
    ///     const DESCRIPTION: &'static str = "target";
    /// }
    ///
    /// impl textkind::TryConvertFrom<SourceKind> for TargetKind {
    ///
    ///     type Error = textkind::Error<Self>;
    ///
    ///     fn try_convert_from<D>(
    ///         source: textkind::Text<SourceKind, D>,
    ///     ) -> textkind::ConvertResult<
    ///         SourceKind,
    ///         TargetKind,
    ///         D,
    ///         Self::Error,
    ///     >
    ///     where
    ///         D: textkind::Dynamic,
    ///     {
    ///         source.try_kind_transition().map_err(Into::into)
    ///     }
    /// }
    ///
    /// let source: textkind::Text<SourceKind, String> =
    ///     textkind::Text::try_from_string("foo".to_string())?;
    ///
    /// let target: textkind::Text<TargetKind, _> =
    ///     textkind::Text::try_convert_from(source)?;
    ///
    /// println!("target value is {}", target);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_convert_from<K2>(other: Text<K2, D>) -> ConvertResult<K2, K, D, K::Error>
    where
        K2: Kind,
        K: TryConvertFrom<K2>,
    {
        K::try_convert_from(other)
    }
    
    /// Try to convert to another text kind via the `TryConvertFrom` trait.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// struct SourceKind;
    /// struct TargetKind;
    ///
    /// impl textkind::Kind for SourceKind {
    ///     type Check = textkind::check::Identifier;
    ///     const DESCRIPTION: &'static str = "source";
    /// }
    ///
    /// impl textkind::Kind for TargetKind {
    ///     type Check = textkind::check::Title;
    ///     const DESCRIPTION: &'static str = "target";
    /// }
    ///
    /// impl textkind::TryConvertFrom<SourceKind> for TargetKind {
    ///
    ///     type Error = textkind::Error<Self>;
    ///
    ///     fn try_convert_from<D>(
    ///         source: textkind::Text<SourceKind, D>,
    ///     ) -> textkind::ConvertResult<
    ///         SourceKind,
    ///         TargetKind,
    ///         D,
    ///         Self::Error,
    ///     >
    ///     where
    ///         D: textkind::Dynamic,
    ///     {
    ///         source.try_kind_transition().map_err(Into::into)
    ///     }
    /// }
    ///
    /// let source: textkind::Text<SourceKind, String> =
    ///     textkind::Text::try_from_string("foo".to_string())?;
    ///
    /// let target: textkind::Text<TargetKind, _> =
    ///     source.try_convert_into()?;
    ///
    /// println!("target value is {}", target);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_convert_into<K2>(self) -> ConvertResult<K, K2, D, K2::Error>
    where
        K2: Kind,
        K2: TryConvertFrom<K>,
    {
        K2::try_convert_from(self)
    }

    /// Get a `&str` view from the text value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// let text: textkind::Title<String> =
    ///     textkind::Title::try_from_str("foo")?;
    ///
    /// assert_eq!(text.as_str(), "foo");
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_str(&self) -> &str { self.data.as_str() }

    /// Turn the text into a `String`.
    ///
    /// Depending on the dynamic storage this might be extracted without causing an allocation.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// let text: textkind::Title<String> =
    ///     textkind::Title::try_from_str("foo")?;
    ///
    /// let value = text.into_string();
    /// assert_eq!(&value, "foo");
    /// # Ok(())
    /// # }
    /// ```
    pub fn into_string(self) -> String { self.data.into_string() }

    /// Turn the text into an `std::borrow::Cow<'static, str>`.
    ///
    /// This will return a `std::borrow::Cow::Borrowed(&'static str)` when the stored value is
    /// static and not in dynamic storage.
    ///
    /// Depending on the dynamic storage a non-static value might be extracted without
    /// causing an allocation.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// // store a &'static str
    /// let text: textkind::Title<String> =
    ///     textkind::Title::try_from_static_str("foo")?;
    ///
    /// // retrieve a &'static str
    /// let value = text.into_static_str_cow();
    /// # Ok(())
    /// # }
    /// ```
    pub fn into_static_str_cow(self) -> borrow::Cow<'static, str> {
        self.data.into_static_str_cow()
    }

    /// Extract the dynamic storage value, optionally creating one if the value is static.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    /// use std::sync::Arc;
    ///
    /// let shared_string = Arc::new("foo".to_string());
    ///
    /// // store a shared string
    /// let text = textkind::Title::try_from_dynamic(shared_string)?;
    ///
    /// // extract the shared string
    /// let value = text.into_dynamic();
    ///
    /// assert_eq!(&*value, "foo");
    /// # Ok(())
    /// # }
    /// ```
    pub fn into_dynamic(self) -> D {
        self.data.into_dynamic()
    }

    /// Extract the data value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    /// use std::sync::Arc;
    ///
    /// let shared_string = Arc::new("foo".to_string());
    ///
    /// // store a shared string
    /// let text = textkind::Title::try_from_data(
    ///     textkind::Data::Dynamic(shared_string),
    /// )?;
    ///
    /// // extract the shared string
    /// let value = text.into_data();
    ///
    /// assert_eq!(value.as_str(), "foo");
    /// # Ok(())
    /// # }
    /// ```
    pub fn into_data(self) -> Data<D> { self.data }

    /// Attempt to transition to another kind.
    ///
    /// If both kinds share the same `Check` type you can use the infallible
    /// [`kind_transition`](#method.kind_transition) method.
    ///
    /// # Errors
    ///
    /// Returns an `ErrorWithValue<K>` with the original value when the value is invalid.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// let identifier: textkind::Identifier<String> =
    ///     textkind::Identifier::try_from_str("foo")?;
    ///
    /// let title: textkind::Title<_> = identifier.try_kind_transition()?;
    ///
    /// assert_eq!(title.as_str(), "foo");
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_kind_transition<K2>(self) -> Result<Text<K2, D>, ErrorWithValue<K2, Text<K, D>>>
    where
        K2: Kind,
    {
        let value = error_with_value!(self, K2::Check::check(self.as_str()))?;
        Ok(Text {
            _kind: marker::PhantomData,
            data: value.data,
        })
    }

    /// Transition to another kind with the same `Check` type.
    ///
    /// See [`try_kind_transition`](#method.try_kind_transition) for transitions where the
    /// `Check` type isn't shared.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    ///
    /// struct SourceKind;
    /// struct TargetKind;
    ///
    /// impl textkind::Kind for SourceKind {
    ///     type Check = textkind::check::Title;
    ///     const DESCRIPTION: &'static str = "source";
    /// }
    ///
    /// impl textkind::Kind for TargetKind {
    ///     type Check = textkind::check::Title;
    ///     const DESCRIPTION: &'static str = "target";
    /// }
    ///
    /// let source: textkind::Text<SourceKind, String> =
    ///     textkind::Text::try_from_str("foo")?;
    ///
    /// let target: textkind::Text<TargetKind, _> =
    ///     source.kind_transition();
    ///
    /// assert_eq!(target.as_str(), "foo");
    /// # Ok(())
    /// # }
    /// ```
    pub fn kind_transition<K2, C>(self) -> Text<K2, D>
    where
        K: Kind<Check = C>,
        K2: Kind<Check = C>,
        C: Check,
    {
        Text {
            _kind: marker::PhantomData,
            data: self.data,
        }
    }

    /// Transition to another dynamic storage.
    ///
    /// The text kind will stay the same.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() { example().expect("no errors") }
    /// # fn example() -> Result<(), Box<::std::error::Error>> {
    /// extern crate textkind;
    /// use std::sync::Arc;
    ///
    /// let local: textkind::Title<String> =
    ///     textkind::Title::try_from_str("foo")?;
    ///
    /// let global: textkind::Title<Arc<String>> = local.storage_transition();
    ///
    /// send_check(global);
    ///
    /// fn send_check<T>(value: T) where T: Send + AsRef<str> {
    ///     assert_eq!(value.as_ref(), "foo");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn storage_transition<D2>(self) -> Text<K, D2>
    where
        D2: Dynamic,
    {
        Text {
            _kind: marker::PhantomData,
            data: self.data.convert(),
        }
    }
}

impl<K, D> Clone for Text<K, D>
where
    K: Kind,
    D: Dynamic,
{
    fn clone(&self) -> Self {
        Text {
            _kind: marker::PhantomData,
            data: self.data.clone(),
        }
    }
}

impl<K, D> str::FromStr for Text<K, D>
where
    K: Kind,
    D: Dynamic,
{
    type Err = Error<K>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Text::try_from_str(value)
    }
}

impl<K, D> fmt::Debug for Text<K, D>
where
    K: Kind,
    D: Dynamic + fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Text {{ data: {:?}, .. }}", self.data)
    }
}

impl<K, D> fmt::Display for Text<K, D>
where
    K: Kind,
    D: Dynamic,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmt)
    }
}

impl<K, D> AsRef<str> for Text<K, D>
where
    K: Kind,
    D: Dynamic,
{
    fn as_ref(&self) -> &str { self.as_str() }
}

impl<K, D> Eq for Text<K, D>
where
    K: Kind,
    D: Dynamic,
{}

impl<K, D, T> PartialEq<T> for Text<K, D>
where
    K: Kind,
    D: Dynamic,
    T: AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl<K, D> Ord for Text<K, D>
where
    K: Kind,
    D: Dynamic,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<K, D, T> PartialOrd<T> for Text<K, D>
where
    K: Kind,
    D: Dynamic,
    T: AsRef<str>,
{
    fn partial_cmp(&self, other: &T) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}

impl<K, D> hash::Hash for Text<K, D>
where
    K: Kind,
    D: Dynamic,
{
    fn hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher,
    {
        self.as_str().hash(hasher)
    }
}

impl<K, D> ops::Deref for Text<K, D>
where
    K: Kind,
    D: Dynamic,
{
    type Target = str;

    fn deref(&self) -> &str { self.as_str() }
}
