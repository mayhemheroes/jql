#![deny(clippy::pedantic)]
#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2021_compatibility)]
#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/yamafaktory/jql/a438ab0039faf64c3329fb09bae399ae95601000/jql.svg"
)]

/// Array utilities.
mod array;
/// Parser errors.
pub mod errors;
/// Object utilities.
mod object;
/// Runner utilities.
pub mod runner;
