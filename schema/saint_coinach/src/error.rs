#[derive(thiserror::Error, Debug)]
pub enum Error {
	// TODO: I should probably make the not found errors more data-y, like _what_ wasn't found _where_, etc.
	#[error("Not found: {0}")]
	NotFound(String),

	// TODO: This exposes the fact that we _use_ git, but not the impl details of git2. is that enough? is that too much? I'm not sure.
	#[error("Repository error: {0}")]
	Repository(String),

	#[error("Schema error: {0}")]
	Schema(String),
}

// TODO: aaaaaa i don't knoooow. if kept, doc(hidden)?
impl From<git2::Error> for Error {
	fn from(error: git2::Error) -> Self {
		Error::Repository(error.to_string())
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
