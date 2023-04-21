use super::format::Format;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("source \"{0}\" does not exist")]
	NotFound(String),

	#[error("source file \"{0}\" is unsupported: {1}")]
	UnsupportedSource(String, String),

	#[error("{0} cannot be converted to {1:?}")]
	InvalidConversion(String, Format),

	#[error(transparent)]
	Failure(#[from] anyhow::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
