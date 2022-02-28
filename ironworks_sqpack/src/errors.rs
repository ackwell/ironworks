use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqPackError {
	#[error("Invalid sqpack path \"{0}\"")]
	InvalidPath(String),

	#[error("Unknown {segment_type} \"{segment}\"")]
	UnknownPathSegment {
		segment_type: String,
		segment: String,
	},

	#[error("IO error: {0}")]
	Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, SqPackError>;
