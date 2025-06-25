#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("filesystem")]
	Filesystem(#[source] Box<dyn std::error::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;
