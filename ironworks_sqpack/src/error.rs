use std::io;

use thiserror::Error;

/// An error that occured while working with a SqPack database.
#[derive(Error, Debug)]
pub enum Error {
	/// A requested file path could not be found in the database indices.
	#[error("Could not find file \"{0}\"")]
	NotFound(String),

	// TODO: This might be combineable with UnknownPathSegment, I'm not convinced the seperation is worthwhile
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

	// TODO: this is probably a bit overloaded. maybe split into "couldn't find db" and "db is broken"
	/// Invalid or unexpected information was found while reading database files.
	/// This typically signifies that the configured database is corrupt, missing
	/// files, or has an incorrect path.
	#[error("Invalid database: {0}")]
	InvalidDatabase(String),

	/// An IO error occured.
	#[error("IO error: {0}")]
	Io(#[from] io::Error),
}

#[doc(hidden)]
pub type Result<T> = std::result::Result<T, Error>;
