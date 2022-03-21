#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Invalid resource: {0}")]
	InvalidResource(String),

	#[error("Not found: {0}")]
	NotFound(String),

	// TODO remove?
	#[error(transparent)]
	Downstream(#[from] anyhow::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
