use std::{
	path::PathBuf,
	str,
	sync::{Arc, Mutex},
};

use derivative::Derivative;
use git2::{ErrorCode, Repository};

use crate::{
	error::{Error, ErrorValue, Result},
	schema::{Schema, Sheet},
};

use super::{parse::parse, specifier::Specifier};

/// A single version of the EXDSchema definitions.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Version {
	#[derivative(Debug = "ignore")]
	repository: Arc<Mutex<Repository>>,

	specifier: Specifier,
}

impl Version {
	pub(super) fn new(repository: Arc<Mutex<Repository>>, specifier: Specifier) -> Self {
		Self {
			repository,
			specifier,
		}
	}

	fn sheet(&self, sheet: &str) -> Result<Sheet> {
		let repository = self.repository.lock().unwrap();
		let commit = repository.find_commit(self.specifier.commit)?;

		let path: PathBuf = [
			"Schemas",
			&self.specifier.game_version,
			&format!("{sheet}.yml"),
		]
		.iter()
		.collect();

		let entry = commit
			.tree()?
			.get_path(&path)
			.map_err(|error| match error.code() {
				ErrorCode::NotFound => Error::NotFound(ErrorValue::Other(format!("sheet {sheet}"))),
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
		self.sheet(name)
	}
}
