use std::io;

use thiserror::Error;

/// An error that occured while working with a SqPack database.
#[derive(Error, Debug)]
pub enum Error {
	/// A requested file path could not be found in the database indices.
	#[error("Could not find file \"{0}\"")]
	NotFound(String),

	/// A requested file path was invalid.
	#[error("Invalid path \"{0}\"")]
	InvalidPath(String),

	/// A well-known path segment was requested, but was not configured.
	#[error("Unknown {segment_type} \"{segment}\"")]
	UnknownPathSegment {
		/// The path segment type (i.e. "category").
		segment_type: String,
		/// The requested segment that could not be found.
		segment: String,
	},

	/// Malformed binary data was found while reading database files.
	#[error("Invalid data encountered: {0}")]
	InvalidData(String),

	/// An IO error occured.
	#[error("IO error: {0}")]
	Io(#[from] io::Error),
}

#[doc(hidden)]
pub type Result<T> = std::result::Result<T, Error>;
