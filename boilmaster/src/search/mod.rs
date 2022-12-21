mod error;
mod index;
mod query;
mod search;
mod version;

pub use {
	error::{FieldTypeError, MismatchError, SearchError},
	search::{Config, Search},
};
