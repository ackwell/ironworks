use std::{fmt, io};

/// An error that occured.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
	/// The requested value could not be found.
	#[error("the {0} could not be found")]
	NotFound(ErrorValue),

	/// The requested value did not confirm to expected behavior/shape.
	#[error("the {0} is invalid: {1}")]
	Invalid(ErrorValue, String),

	/// An error occured while working with a resource. This is typically IO-related,
	/// stemming from an inability to read or parse various expected structures.
	/// In most circumstances, recovery from this error is not possible without
	/// re-instantiating ironworks and/or the related module.
	#[error("an error occured while working with the provided resource: {0}")]
	Resource(Box<dyn std::error::Error + Send + Sync>),
}

// Common From<> impls for Error::Resource. This is intentionally not a catch-all.
impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Error::Resource(error.into())
	}
}
impl From<binrw::Error> for Error {
	fn from(error: binrw::Error) -> Self {
		Error::Resource(error.into())
	}
}

// TODO: this could get pretty cluttered with single-purpose values. Is it worth making errorvalue a trait (and making error generic over it?) and letting each feature/file implement its own values? Generic trait might make it really messy to move errors around in the project due to non-matching bounds but hey maybe box dyn?
/// A value associated with an error that occured.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorValue {
	/// A path to a file.
	Path(String),

	/// An Excel sheet.
	#[cfg(feature = "excel")]
	Sheet(String),

	/// An Excel row.
	#[cfg(feature = "exd")]
	Row {
		/// Row ID.
		row: u32,
		/// Sub-row ID.
		subrow: u16,
		/// Row's parent sheet, if known.
		sheet: Option<String>,
	},

	/// A SeString rich text value.
	#[cfg(feature = "sestring")]
	SeString,

	/// A SqPack file.
	#[cfg(feature = "sqpack")]
	File(Vec<u8>),

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
			Self::Path(path) => write!(formatter, "path {path:?}"),

			#[cfg(feature = "excel")]
			Self::Sheet(sheet) => write!(formatter, "Excel sheet {sheet:?}"),

			#[cfg(feature = "exd")]
			Self::Row { row, subrow, sheet } => write!(
				formatter,
				"Excel row {}/{row}:{subrow}",
				sheet.as_deref().unwrap_or("(none)"),
			),

			#[cfg(feature = "sestring")]
			Self::SeString => write!(formatter, "SeString"),

			#[cfg(feature = "sqpack")]
			Self::File(file) => write!(formatter, "SqPack file ({} bytes)", file.len()),

			Self::Other(value) => write!(formatter, "{value}"),
		}
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
