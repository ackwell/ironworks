use std::{fmt, path::PathBuf};

// TODO: non exhaustive?
// TODO: should we have, like, sqpack error, excel error, etc, and then a big daddy Error that combines them?
/// An error that occured.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	/// The requested value could not be found.
	#[error("The {0} could not be found.")]
	NotFound(ErrorValue),

	/// The requested value did not confirm to expected behavior/shape.
	#[error("The {0} is invalid: {1}.")]
	Invalid(ErrorValue, String),

	/// An error occured while woring with a resource. This is typically IO-related,
	/// stemming from an inability to read or parse various expected structures from
	/// the provided reader.
	#[error("An error occured while working with the provided resource: {0}")]
	Resource(Box<dyn std::error::Error + Send + Sync>),
}

// TODO: non exhaustive?
/// A value associated with an error that occured.
#[derive(Debug)]
pub enum ErrorValue {
	/// A filesystem path.
	FilePath(PathBuf),
	/// A path within the SqPack package repositories.
	SqpackPath(String),

	/// A value not otherwise represented by the above variants.
	Other(String),
}

impl fmt::Display for ErrorValue {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::FilePath(path) => write!(formatter, "file {path:?}"),
			Self::SqpackPath(path) => write!(formatter, "SqPack path \"{path}\""),

			Self::Other(value) => write!(formatter, "{value}"),
		}
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
