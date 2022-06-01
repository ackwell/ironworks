use std::{
	hash::Hash,
	io::{Read, Seek},
};

use crate::error::Result;

/// Resource adapter to fetch information and data on request for a SqPack instance.
pub trait Resource {
	/// Metadata associated with a SqPack path that can be used to identify archive
	/// files the path can be retrieved from.
	type PathMetadata: Clone + Eq + Hash;

	/// Retrieve the `PathMetadata` for a given SqPack path, or `None` if the path
	/// is invalid or does not conform to valid formatting for this resource.
	fn path_metadata(&self, path: &str) -> Option<Self::PathMetadata>;

	/// Get the version string for a given repository.
	fn version(&self, path_metadata: &Self::PathMetadata) -> Result<String>;

	/// The type of an index resource.
	type Index: Read + Seek;
	/// Fetches the specified index resource.
	fn index(&self, path_metadata: &Self::PathMetadata, chunk: u8) -> Result<Self::Index>;

	/// The type of an index2 resource.
	type Index2: Read + Seek;
	/// Fetches the specified index2 resource.
	fn index2(&self, path_metadata: &Self::PathMetadata, chunk: u8) -> Result<Self::Index2>;

	/// The type of a dat resource.
	type Dat: Read + Seek;
	/// Fetches the specified dat resource.
	fn dat(&self, path_metadata: &Self::PathMetadata, chunk: u8, dat: u8) -> Result<Self::Dat>;
}
