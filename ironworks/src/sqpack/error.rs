use std::io;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("requested file could not be found")]
	FileNotFound,

	#[error("provided file path is invalid: {0}")]
	PathInvalid(String),

	#[error("file is empty or missing header")]
	FileIncomplete(Vec<u8>),

	#[error("malformed data encountered")]
	Malformed(#[source] Box<dyn std::error::Error>),

	#[error("I/O error encountered")]
	Io(#[from] io::Error),
}

impl From<binrw::Error> for Error {
	fn from(error: binrw::Error) -> Self {
		match error {
			binrw::Error::Io(inner) => Self::Io(inner),
			error => Self::Malformed(error.into()),
		}
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
