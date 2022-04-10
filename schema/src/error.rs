use core::fmt;

/// An error that occured.
#[derive(thiserror::Error, Debug)]
pub enum Error {}

/// A value associated with an error.
#[derive(Debug)]
pub enum ErrorValue {
	/// A value not represented by other variants.
	///
	/// `ErrorValue`s of the `Other` type should only be match`ed on with a wildcard
	/// (`_`) pattern. Values represented by `Other` may be promoted to a new variant
	/// in future versions.
	Other(String),
}

impl fmt::Display for ErrorValue {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Other(value) => write!(formatter, "{value}"),
		}
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
