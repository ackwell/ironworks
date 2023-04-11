#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Failure(anyhow::Error),
}

// Implement From traits for common search-related failures that can be marked as a full failure.
macro_rules! impl_to_failure {
	($source:ty) => {
		impl From<$source> for Error {
			fn from(value: $source) -> Self {
				Self::Failure(value.into())
			}
		}
	};
}

// TODO: Consider if any of these need to split out some of the error types into not-failure.
impl_to_failure!(ironworks::Error);
impl_to_failure!(std::io::Error);
impl_to_failure!(tantivy::TantivyError);
impl_to_failure!(tantivy::directory::error::OpenDirectoryError);
impl_to_failure!(tantivy::directory::error::OpenReadError);
impl_to_failure!(tokio::task::JoinError);

pub type Result<T, E = Error> = std::result::Result<T, E>;
