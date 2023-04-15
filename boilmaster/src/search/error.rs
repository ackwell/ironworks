#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("invalid field value on {}: could not coerce {} value to {}", .0.field, .0.got, .0.expected)]
	FieldType(FieldTypeError),

	#[error("malformed search query: {0}")]
	MalformedQuery(String),

	/// The provided query cannot be mapped onto the sheet schema.
	#[error("query <-> schema mismatch on {}: {}", .0.field, .0.reason)]
	QuerySchemaMismatch(MismatchError),

	/// The provided query cannot be normalized in terms of the game data.
	#[error("query <-> game mismatch on {}: {}", .0.field, .0.reason)]
	QueryGameMismatch(MismatchError),

	/// The sheet schema in use does not map cleanly to the search index (and hence game data).
	#[error("schema <-> game mismatch on {}: {}", .0.field, .0.reason)]
	SchemaGameMismatch(MismatchError),

	#[error(transparent)]
	Failure(anyhow::Error),
}

#[derive(Debug)]
pub struct FieldTypeError {
	pub(super) field: String,
	pub(super) expected: String,
	pub(super) got: String,
}

#[derive(Debug)]
pub struct MismatchError {
	pub(super) field: String,
	pub(super) reason: String,
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
impl_to_failure!(anyhow::Error);
impl_to_failure!(ironworks::Error);
impl_to_failure!(serde_json::Error);
impl_to_failure!(std::io::Error);
impl_to_failure!(tantivy::TantivyError);
impl_to_failure!(tantivy::directory::error::OpenDirectoryError);
impl_to_failure!(tantivy::directory::error::OpenReadError);
impl_to_failure!(tokio::task::JoinError);

pub type Result<T, E = Error> = std::result::Result<T, E>;
