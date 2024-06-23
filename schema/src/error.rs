use core::fmt;

/// An error that occured.
#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
	/// The requested value could not be found.
	#[error("The {0} could not be found.")]
	NotFound(ErrorValue),

	/// An error occured while generating a schema.
	#[error("Schema error: {0}.")]
	Schema(String),

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
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorValue {
	#[cfg(any(feature = "saint_coinach", feature = "exdschema"))]
	/// A version of a schema.
	Version(String),

	/// A sheet by name.
	Sheet(String),

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
			#[cfg(any(feature = "saint_coinach", feature = "exdschema"))]
			Self::Version(value) => write!(formatter, "version {value}"),

			Self::Sheet(name) => write!(formatter, "sheet {name}"),
			Self::Other(value) => write!(formatter, "{value}"),
		}
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
