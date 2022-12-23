mod error;
mod index;
mod search;
mod version;

// TODO: I don't like exposing this whole thing but it's the easiest way to not have conflicting name requirements between internal and external surfaces.
pub mod query;

pub use {
	error::{FieldTypeError, MismatchError, SearchError},
	search::{Config, Search},
};
