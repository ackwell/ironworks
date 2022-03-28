use std::io::{Read, Seek};

pub trait Resource {
	fn path_metadata<'a>(&self, path: &'a str) -> Option<(&'a str, &'a str)>;

	type Index: Read + Seek;
	fn index(&self, repository: &str, category: &str, chunk: u8) -> Self::Index;

	type Index2: Read + Seek;
	fn index2(&self, repository: &str, category: &str, chunk: u8) -> Self::Index2;

	type Dat: Read + Seek;
	fn dat(&self, repository: &str, category: &str, chunk: u8) -> Self::Dat;
}
