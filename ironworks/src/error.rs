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
	#[cfg(feature = "sqpack")]
	FilePath(PathBuf),

	/// A path within the SqPack package repositories.
	#[cfg(feature = "sqpack")]
	SqpackPath(String),

	/// An Excel sheet.
	#[cfg(feature = "excel")]
	Sheet(String),

	/// An Excel row.
	#[cfg(feature = "excel")]
	Row {
		/// Row ID.
		row: u32,
		/// Sub-row ID.
		subrow: u16,
		/// Row's parent sheet.
		sheet: String,
	},

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
			#[cfg(feature = "sqpack")]
			Self::FilePath(path) => write!(formatter, "file {path:?}"),

			#[cfg(feature = "sqpack")]
			Self::SqpackPath(path) => write!(formatter, "SqPack path \"{path}\""),

			#[cfg(feature = "excel")]
			Self::Sheet(sheet) => write!(formatter, "Excel sheet \"{sheet}\""),

			#[cfg(feature = "excel")]
			Self::Row { row, subrow, sheet } => write!(formatter, "Excel row {sheet}/{row}:{subrow}"),

			Self::Other(value) => write!(formatter, "{value}"),
		}
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
