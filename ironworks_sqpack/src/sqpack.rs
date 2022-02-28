use std::{collections::HashMap, path::PathBuf};

use crate::{dat_reader::DatReader, errors::SqPackError};

pub struct Repository {
	pub name: String,
	pub id: u8,
	pub path: PathBuf,
}

pub struct Category {
	pub name: String,
	pub id: u8,
}

// TODO: this should probably be in own file

#[derive(Debug)]
pub struct SqPack {
	pub repositories: HashMap<String, PathBuf>,
	pub categories: HashMap<String, u8>,

	pub default_repository: String,
}

impl SqPack {
	pub fn temp_test(&self, sqpack_path: &str) -> Result<(), SqPackError> {
		let path = self.parse_path(sqpack_path)?;

		let repository_path = self.repositories.get(&path.repository).ok_or_else(|| {
			SqPackError::UnknownRepository {
				path: path.path.clone(),
				repository: path.repository.clone(),
			}
		})?;

		let category_id =
			self.categories
				.get(&path.category)
				.ok_or_else(|| SqPackError::UnknownCategory {
					path: path.path.clone(),
					category: path.category.clone(),
				})?;

		println!("repo: {:?}, cat: {}", repository_path, category_id);

		// TODO: cache readers
		let reader = DatReader::new(
			Repository {
				id: 0,
				name: path.repository,
				path: repository_path.to_owned(),
			},
			Category {
				id: *category_id,
				name: path.category,
			},
		);

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
