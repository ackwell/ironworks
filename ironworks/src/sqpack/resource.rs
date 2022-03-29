use std::io::{Read, Seek};

/// Resource adapter to fetch information and data on request for a SqPack instance.
pub trait Resource {
	/// Retrieve the `(repository, category)` for a given SqPack path, or `None` if
	/// the path is invalid or does not conform to valid formatting for this resource.
	fn path_metadata<'a>(&self, path: &'a str) -> Option<(&'a str, &'a str)>;

	/// The type of an index resource.
	type Index: Read + Seek;
	/// Fetches the specified index resource.
	fn index(&self, repository: &str, category: &str, chunk: u8) -> Self::Index;

	/// The type of an index2 resource.
	type Index2: Read + Seek;
	/// Fetches the specified index2 resource.
	fn index2(&self, repository: &str, category: &str, chunk: u8) -> Self::Index2;

	// TODO: this will probably need a dat no. param too
	/// The type of a dat resource.
	type Dat: Read + Seek;
	/// Fetches the specified dat resource.
	fn dat(&self, repository: &str, category: &str, chunk: u8) -> Self::Dat;
}
