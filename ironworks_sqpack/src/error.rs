use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Could not find file \"{0}\"")]
	NotFound(String),

	#[error("Invalid path \"{0}\"")]
	InvalidPath(String),

	#[error("Unknown {segment_type} \"{segment}\"")]
	UnknownPathSegment {
		segment_type: String,
		segment: String,
	},

	#[error("Invalid data encountered: {0}")]
	InvalidData(String),

	#[error("IO error: {0}")]
	Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
