use core::fmt;

/// An error that occured.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	/// The requested value could not be found.
	#[error("The {0} could not be found.")]
	NotFound(ErrorValue),

	/// An error occured while working with a git repository.
	#[cfg(feature = "git2")]
	#[error("{0}")]
	Repository(String),
}

#[cfg(feature = "git2")]
impl From<git2::Error> for Error {
	fn from(error: git2::Error) -> Self {
		Error::Repository(error.to_string())
	}
}

/// A value associated with an error.
#[derive(Debug)]
pub enum ErrorValue {
	/// A value not represented by other variants.
	///
	/// `ErrorValue`s of the `Other` type should only be `match`ed on with a wildcard
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
