use std::{collections::HashMap, path::Path};
use thiserror::Error;

#[derive(Debug)]
pub struct SqPack<'a> {
	pub repositories: HashMap<&'a str, &'a Path>,
	pub categories: HashMap<&'a str, u8>,

	pub default_repository: &'a str,
}

// TODO: this should probably be in own file
#[derive(Error, Debug)]
pub enum SqPackError {
	#[error("invalid sqpack path \"{0}\"")]
	InvalidPath(String),
}

impl SqPack<'_> {
	pub fn temp_test(&self, sqpack_path: &str) -> Result<(), SqPackError> {
		// TODO: Look into itertools or something?
		let split = sqpack_path.splitn(3, '/').take(2).collect::<Vec<&str>>();
		let (category, mut repository) = match split[..] {
			[category, repository] => (category, repository),
			_ => return Err(SqPackError::InvalidPath(sqpack_path.to_string())),
		};

		if !self.repositories.contains_key(repository) {
			repository = self.default_repository
		}

		println!(
			"test; cat {:?} repo {:?} path {:?}",
			category, repository, sqpack_path
		);

		return Ok(());
	}
}
