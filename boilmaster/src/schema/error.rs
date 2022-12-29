#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("unknown schema source \"{0}\"")]
	UnknownSource(String),

	#[error("invalid schema version \"{0}\"")]
	InvalidVersion(String),

	#[error(transparent)]
	Failure(#[from] anyhow::Error),
}
