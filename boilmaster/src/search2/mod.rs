mod error;
#[path = "query/mod.rs"]
mod internal_query;
mod search;
mod tantivy;

pub use {
	error::{Error, FieldTypeError, MismatchError},
	internal_query::pre as query,
	search::{Config, Search},
};
