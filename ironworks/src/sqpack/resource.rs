use std::io::{Read, Seek};

use crate::error::Result;

/// Resource adapter to fetch information and data on request for a SqPack instance.
pub trait Resource {
	/// Retrieve the `(repository, category)` for a given SqPack path, or `None` if
	/// the path is invalid or does not conform to valid formatting for this resource.
	fn path_metadata(&self, path: &str) -> Option<(u8, u8)>;

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

	/// The type of a dat resource.
	type Dat: Read + Seek + Send + Sync;
	/// Fetches the specified dat resource.
	fn dat(&self, repository: u8, category: u8, chunk: u8, dat: u8) -> Result<Self::Dat>;
}
