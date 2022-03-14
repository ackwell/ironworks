#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Invalid resource: {0}")]
	InvalidResource(String),

	#[error("Not found: {0}")]
	NotFound(String),

	#[error(transparent)]
	Downstream(anyhow::Error),
}

// TODO: the below is no longer correct. remove?
// Due to the nature of the ExcelResource trait, it's expected that an anyhow::Error
// returned by a resource function could be a first-party error. To avoid blindly
// bubbling our own errors up as a Downstream, we're manually implementing From
// here and trying to downcast to ourselves - only wrapping in Downstream if that
// is not possible.
impl From<anyhow::Error> for Error {
	fn from(error: anyhow::Error) -> Self {
		match error.downcast::<Error>() {
			Ok(error) => error,
			Err(error) => Error::Downstream(error),
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;
