use std::{
	collections::hash_map::Entry,
	str,
	sync::{Arc, Mutex},
};

use derivative::Derivative;
use git2::{ErrorCode, Repository};

use crate::{
	error::{Error, ErrorValue, Result},
	schema::{Schema, Sheet},
};

use super::{parse::parse, provider::SheetCache, specifier::Specifier};

/// A single version of the EXDSchema definitions.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Version {
	#[derivative(Debug = "ignore")]
	repository: Arc<Mutex<Repository>>,

	#[derivative(Debug = "ignore")]
	cache: SheetCache,

	specifier: Specifier,
}

impl Version {
	pub(super) fn new(
		repository: Arc<Mutex<Repository>>,
		cache: SheetCache,
		specifier: Specifier,
	) -> Self {
		Self {
			repository,
			cache,
			specifier,
		}
	}

	fn cached_sheet(&self, sheet: &str) -> Result<Sheet> {
		match &self.cache {
			None => self.sheet(sheet),
			Some(mutex) => {
				let mut cache = mutex.lock().expect("poisoned");
				match cache.entry((self.specifier.clone(), sheet.into())) {
					Entry::Occupied(entry) => entry.get().clone(),
					Entry::Vacant(entry) => {
						let result = match self.sheet(sheet) {
							result @ Ok(_) | result @ Err(Error::NotFound(_)) => result,
							Err(other_error) => return Err(other_error),
						};
						entry.insert(result).clone()
					}
				}
			}
		}
	}

	fn sheet(&self, sheet: &str) -> Result<Sheet> {
		let repository = self.repository.lock().unwrap();
		let commit = repository.find_commit(self.specifier.commit())?;
		let path = self.specifier.sheet_path(sheet);

		let entry = commit
			.tree()?
			.get_path(&path)
			.map_err(|error| match error.code() {
				ErrorCode::NotFound => Error::NotFound(ErrorValue::Sheet(sheet.into())),
				_ => Error::from(error),
			})?;

		let blob = entry
			.to_object(&repository)?
			.into_blob()
			.map_err(|object| {
				Error::Repository(format!(
					"expected blob for {sheet} sheet schema, got {:?}",
					object.kind()
				))
			})?;

		parse(blob.content())
	}
}

impl Schema for Version {
	fn sheet(&self, name: &str) -> Result<Sheet> {
		self.cached_sheet(name)
	}
}
