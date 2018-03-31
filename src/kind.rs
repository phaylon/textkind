//! Predefined text kinds.
//!
//! It is proably more convenient to use the text type aliases exported from the top level
//! instead of directly using these kinds. That way it is possible to say `Title<String>`
//! instead of `Text<Title, String>`.
//!
//! See the `check` module for predefined check types.
//!
//! See the `Check` trait for an example on implementing custom checks.
//!
//! See the `Kind` trait for an example on how to associate a check with a custom kind.

use check;

/// Text kind representing a title.
///
/// This kind combines the predefined `Title` check with the `MaxBytes512` check.
#[allow(missing_debug_implementations)]
pub struct Title {
    _unconstructable: ::Void,
}

impl ::Kind for Title {

    type Check = check::And<check::MaxBytes512, check::Title>;

    const DESCRIPTION: &'static str = "title";
}

/// Text kind representing an identifier.
///
/// This kind combines the predefined `Identifier` check with the `MaxBytes512` check.
#[allow(missing_debug_implementations)]
pub struct Identifier {
    _unconstructable: ::Void,
}

impl ::Kind for Identifier {

    type Check = check::And<check::MaxBytes512, check::Identifier>;

    const DESCRIPTION: &'static str = "identifier";
}

/// Text kind representing a relaxed identifier.
///
/// This kind combines the predefined `IdentifierLax` check with the `MaxBytes512` check.
#[allow(missing_debug_implementations)]
pub struct IdentifierLax {
    _unconstructable: ::Void,
}

impl ::Kind for IdentifierLax {

    type Check = check::And<check::MaxBytes512, check::IdentifierLax>;

    const DESCRIPTION: &'static str = "identifier";
}

