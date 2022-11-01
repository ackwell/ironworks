use std::io;

use crate::{error::Result, sqpack};

#[derive(Debug)]
pub struct Version {}

impl Version {
	pub(super) fn new() -> Self {
		Self {}
	}
}

impl sqpack::Resource for Version {
	fn path_metadata(&self, path: &str) -> Option<(u8, u8)> {
		todo!("path_metadata({path})")
	}

	fn version(&self, repository: u8) -> Result<String> {
		todo!("version({repository})")
	}

	type Index = io::Empty;
	fn index(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index> {
		todo!("index({repository}, {category}, {chunk})")
	}

	type Index2 = io::Empty;
	fn index2(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index2> {
		todo!("index2({repository}, {category}, {chunk})")
	}

	type File = io::Empty;
	fn file(&self, repository: u8, category: u8, location: sqpack::Location) -> Result<Self::File> {
		todo!("file({repository}, {category}, {location:?})")
	}
}
