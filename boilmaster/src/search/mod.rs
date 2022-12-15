mod error;
mod index;
mod ingest;
mod query;
mod search;
mod version;

pub use {
	error::{FieldTypeError, SearchError},
	search::{Config, Search},
};
