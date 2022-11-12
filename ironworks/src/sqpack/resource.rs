use std::io::{Read, Seek};

use crate::error::Result;

use super::index::Location;

/// Resource adapter to fetch information and data on request for a SqPack instance.
pub trait Resource {
	/// Get the version string for a given repository.
	fn version(&self, repository: u8) -> Result<String>;

	/// The type of an index resource.
	type Index: Read + Seek;
	/// Fetches the specified index resource.
	fn index(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index>;

	/// The type of an index2 resource.
	type Index2: Read + Seek;
	/// Fetches the specified index2 resource.
	fn index2(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index2>;

	/// The type of a file reader resource.
	type File: Read + Seek;
	/// Fetch a reader for the specified file from a dat container.
	fn file(&self, repository: u8, category: u8, location: Location) -> Result<Self::File>;
}
