use std::{
	cell::RefCell,
	collections::{hash_map::Entry, HashMap},
	path::PathBuf,
	rc::Rc,
};

use crate::{
	dat_reader::DatReader,
	error::{Error, Result},
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
pub struct SqPack<'a> {
	default_repository: String,
	repositories: HashMap<String, Repository>,
	categories: HashMap<String, Category>,

	reader_cache: RefCell<HashMap<String, Rc<DatReader<'a>>>>,
}

impl<'a> SqPack<'a> {
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

			reader_cache: RefCell::new(HashMap::new()),
		};
	}

	pub fn read_file(&'a self, raw_sqpack_path: &str) -> Result<Vec<u8>> {
		let sqpack_path = raw_sqpack_path.to_lowercase();
		let reader = self.get_reader(&sqpack_path)?;
		return reader.read_file(&sqpack_path);
	}

	fn get_reader(&'a self, sqpack_path: &str) -> Result<Rc<DatReader>> {
		// TODO: maybe try_borrow_mut?
		let mut cache = self.reader_cache.borrow_mut();

		// Check if we have a reader for the given metadata, and return it if we do.
		let (category_name, repository_name) = self.parse_segments(sqpack_path)?;
		let vacant_entry = match cache.entry(format!("{}:{}", category_name, repository_name)) {
			Entry::Occupied(entry) => return Ok(entry.get().clone()),
			Entry::Vacant(entry) => entry,
		};

		// No existing reader found, build a new one and store in the cache.
		let repository = self.get_repository(repository_name)?;
		let category = self.get_category(category_name)?;
		let reader = Rc::new(DatReader::new(repository, category)?);

		return Ok(vacant_entry.insert(reader).clone());
	}

	fn parse_segments<'b>(&self, path: &'b str) -> Result<(&'b str, &'b str)> {
		// TODO: consider itertools or similar if we find this pattern a few times
		let split = path.splitn(3, '/').take(2).collect::<Vec<_>>();
		return match split[..] {
			[category_name, repository_name] => Ok((category_name, repository_name)),
			_ => Err(Error::InvalidPath(path.to_owned())),
		};
	}

	fn get_repository(&self, repository_name: &str) -> Result<&Repository> {
		return self
			.repositories
			.get(repository_name)
			.or_else(|| self.repositories.get(&self.default_repository))
			.ok_or_else(|| Error::UnknownPathSegment {
				segment_type: String::from("repository"),
				segment: repository_name.to_owned(),
			});
	}

	fn get_category(&self, category_name: &str) -> Result<&Category> {
		return self
			.categories
			.get(category_name)
			.ok_or_else(|| Error::UnknownPathSegment {
				segment_type: String::from("category"),
				segment: category_name.to_owned(),
			});
	}
}
