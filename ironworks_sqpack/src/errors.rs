use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqPackError {
	#[error("invalid sqpack path \"{0}\"")]
	InvalidPath(String),

	#[error("unknown {segment_type} \"{segment}\"")]
	UnknownPathSegment {
		segment_type: String,
		segment: String,
	},
}

pub type Result<T> = std::result::Result<T, SqPackError>;
