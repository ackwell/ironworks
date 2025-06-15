#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("filesystem error encountered")]
	Filesystem(#[source] Box<dyn std::error::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;
