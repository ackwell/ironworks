mod error;
mod index;
mod query;
mod search;
mod version;

pub use {
	error::{FieldTypeError, SchemaMismatchError, SearchError},
	search::{Config, Search},
};
