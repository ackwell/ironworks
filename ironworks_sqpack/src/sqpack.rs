use std::{collections::HashMap, path::Path};

#[derive(Debug)]
pub struct SqPack<'a> {
	pub repositories: HashMap<&'a str, &'a Path>,
	pub categories: HashMap<&'a str, u8>,

	pub default_repository: &'a str,
}

impl SqPack<'_> {
	pub fn temp_test(&self, sqpack_path: &str) {
		// TODO: Look into itertools or something?
		// TODO: error handling that isn't just unwrap memes
		let mut split = sqpack_path.splitn(3, '/');
		let category_name = split.next().unwrap();
		let mut repository_name = split.next().unwrap();

		if !self.repositories.contains_key(repository_name) {
			repository_name = self.default_repository
		}

		println!(
			"test; cat {:?} repo {:?} path {:?}",
			category_name, repository_name, sqpack_path
		);
	}
}
