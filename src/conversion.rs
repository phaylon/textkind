
use std::error;
use std::fmt;

/// Trait for converting between different text kinds.
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
pub trait ConvertFrom<K>: ::Kind + Sized
where
    K: ::Kind
{
    /// Convert from one kind of text into another while keeping the same dynamic storage
    /// type.
    ///
    /// # Panics
    ///
    /// Since this usually constructs a new text kind from an existing one, a call to this
    /// may run assertions that may panic.
    fn convert_from<D>(text: ::Text<K, D>) -> ::Text<Self, D>
    where
        D: ::Dynamic;
}

/// Signals a conversion error.
///
/// Convenience `From` conversions from `ErrorWithValue<TargetKind, Text<SourceKind, D>>` to
/// `ConvertError<SourceKind, D, Error<TargetKind>>` and vice-versa are provided.
///
/// The type parameters are as follows:
///
/// * `K` is the source text kind.
/// * `D` is the dynamic storage.
/// * `E` is the error.
pub struct ConvertError<K, D, E>(pub E, pub ::Text<K, D>);

impl<K, D, E> error::Error for ConvertError<K, D, E>
where
    K: ::Kind,
    D: ::Dynamic,
    E: error::Error,
    ::Text<K, D>: fmt::Debug,
{
    fn description(&self) -> &str { "conversion error" }
}

impl<K, D, E> fmt::Debug for ConvertError<K, D, E>
where
    K: ::Kind,
    D: ::Dynamic,
    E: fmt::Debug,
    ::Text<K, D>: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "ConvertError({:?}, {:?})", self.0, self.1)
    }
}

impl<K, D, E> fmt::Display for ConvertError<K, D, E>
where
    K: ::Kind,
    D: ::Dynamic,
    E: fmt::Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl<K, D, E> Eq for ConvertError<K, D, E>
where
    K: ::Kind,
    D: ::Dynamic,
    E: Eq,
    ::Text<K, D>: Eq,
{}

impl<K, D, E> PartialEq for ConvertError<K, D, E>
where
    K: ::Kind,
    D: ::Dynamic,
    E: PartialEq,
    ::Text<K, D>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<K1, K2, D> From<::ErrorWithValue<K2, ::Text<K1, D>>>
for ConvertError<K1, D, ::Error<K2>>
where
    K1: ::Kind,
    K2: ::Kind,
    D: ::Dynamic,
{
    fn from(error_with_value: ::ErrorWithValue<K2, ::Text<K1, D>>) -> Self {
        let (error, value) = error_with_value.split();
        ConvertError(error, value)
    }
}

impl<K1, K2, D> From<ConvertError<K1, D, ::Error<K2>>>
for ::ErrorWithValue<K2, ::Text<K1, D>>
where
    K1: ::Kind,
    K2: ::Kind,
    D: ::Dynamic,
{
    fn from(convert_error: ConvertError<K1, D, ::Error<K2>>) -> Self {
        convert_error.0.with_value(convert_error.1)
    }
}

/// Convenience alias for conversion results.
///
/// The type parameters are as follows:
///
/// * `K1` is the source text kind.
/// * `K2` is the target text kind.
/// * `D` is the dynamic storage.
/// * `E` is the error.
pub type ConvertResult<K1, K2, D, E> = Result<
    ::Text<K2, D>,
    ConvertError<K1, D, E>,
>;

/// Trait for fallibly converting between different text kinds.
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
/// let target: textkind::Text<TargetKind, _> = source
///     .try_convert_into()?;
///
/// println!("target value is {}", target);
/// # Ok(())
/// # }
/// ```
pub trait TryConvertFrom<K>: ::Kind + Sized
where
    K: ::Kind,
{
    /// The error type communicating what error occured.
    type Error;

    /// Attempt conversion from another kind.
    fn try_convert_from<D>(text: ::Text<K, D>) -> ConvertResult<K, Self, D, Self::Error>
    where
        D: ::Dynamic;
}

