use std::{collections::HashMap, path::PathBuf};

use crate::{dat_reader::DatReader, errors::SqPackError};

#[derive(Debug)]
pub struct Repository {
	pub name: String,
	pub id: u8,
	pub path: PathBuf,
}

#[derive(Debug)]
pub struct Category {
	pub name: String,
	pub id: u8,
}

// TODO: this should probably be in own file

#[derive(Debug)]
pub struct SqPack {
	repositories: HashMap<String, Repository>,
	categories: HashMap<String, Category>,

	pub default_repository: String,
}

impl SqPack {
	pub fn new(
		default_repository: String,
		repositories: impl IntoIterator<Item = Repository>,
		categories: impl IntoIterator<Item = Category>,
	) -> Self {
		return SqPack {
			default_repository,

			repositories: repositories
				.into_iter()
				.map(|repository| (repository.name.to_owned(), repository))
				.collect(),

			categories: categories
				.into_iter()
				.map(|category| (category.name.to_owned(), category))
				.collect(),
		};
	}

	// pub fn test(&mut self, thing: String) -> &mut Self {
	// 	self.default_repository = thing;
	// 	return self;
	// }

	pub fn temp_test(&self, sqpack_path: &str) -> Result<(), SqPackError> {
		let path = self.parse_path(sqpack_path)?;

		let repository = self.repositories.get(&path.repository).ok_or_else(|| {
			SqPackError::UnknownRepository {
				path: path.path.clone(),
				repository: path.repository.clone(),
			}
		})?;

		let category =
			self.categories
				.get(&path.category)
				.ok_or_else(|| SqPackError::UnknownCategory {
					path: path.path.clone(),
					category: path.category.clone(),
				})?;

		// TODO: cache readers
		let reader = DatReader::new(repository, category);

		let exlt = String::from_utf8(reader.read_file(sqpack_path)).unwrap();

		println!("EXLT: {}", exlt);

		return Ok(());
	}

	fn parse_path(&self, sqpack_path: &str) -> Result<SqPackPath, SqPackError> {
		// TODO: Look into itertools or something?
		let lower = sqpack_path.to_lowercase();
		let split = lower.splitn(3, '/').take(2).collect::<Vec<&str>>();
		let (category, mut repository) = match split[..] {
			[category, repository] => (category, repository),
			_ => return Err(SqPackError::InvalidPath(sqpack_path.to_string())),
		};

		if !self.repositories.contains_key(repository) {
			repository = &self.default_repository
		}

		return Ok(SqPackPath {
			category: String::from(category),
			repository: String::from(repository),
			path: lower,
		});
	}
}

// TODO: probs should call this path and namespace on consume
// TODO: I mean realistically this can just be an internal tuple?
#[derive(Debug)]
pub struct SqPackPath {
	path: String,
	category: String,
	repository: String,
}
