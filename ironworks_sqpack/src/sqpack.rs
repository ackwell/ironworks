use std::{collections::HashMap, path::PathBuf};

use crate::{
	dat_reader::DatReader,
	errors::{Result, SqPackError},
};

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

#[derive(Debug)]
pub struct SqPack {
	repositories: HashMap<String, Repository>,
	categories: HashMap<String, Category>,

	default_repository: String,
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

	pub fn read_file(&self, sqpack_path: &str) -> Result<Vec<u8>> {
		// Get the category and repository metadata
		let lower = sqpack_path.to_lowercase();
		let (category_name, repository_name) = self.parse_segments(&lower)?;

		let repository = self.get_repository(repository_name)?;
		let category = self.get_category(category_name)?;

		// TODO: cache
		let reader = DatReader::new(repository, category);

		return reader.read_file(sqpack_path);
	}

	fn parse_segments<'a>(&self, path: &'a str) -> Result<(&'a str, &'a str)> {
		// TODO: consider itertools or similar if we find this pattern a few times
		let split = path.splitn(3, '/').take(2).collect::<Vec<_>>();
		return match split[..] {
			[category_name, repository_name] => Ok((category_name, repository_name)),
			_ => Err(SqPackError::InvalidPath(path.to_owned())),
		};
	}

	fn get_repository(&self, repository_name: &str) -> Result<&Repository> {
		return self
			.repositories
			.get(repository_name)
			.or_else(|| self.repositories.get(&self.default_repository))
			.ok_or_else(|| SqPackError::UnknownPathSegment {
				segment_type: String::from("repository"),
				segment: repository_name.to_owned(),
			});
	}

	fn get_category(&self, category_name: &str) -> Result<&Category> {
		return self
			.categories
			.get(category_name)
			.ok_or_else(|| SqPackError::UnknownPathSegment {
				segment_type: String::from("category"),
				segment: category_name.to_owned(),
			});
	}
}
