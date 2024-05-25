use std::{path::PathBuf, str, sync::Arc};

use git2::{ErrorCode, Repository};

use crate::{
	error::{Error, ErrorValue, Result},
	schema::{Schema, Sheet},
};

use super::{parse::parse, specifier::Specifier};

pub struct Version {
	repository: Arc<Repository>,

	specifier: Specifier,
}

impl Version {
	pub(super) fn new(repository: Arc<Repository>, specifier: Specifier) -> Self {
		Self {
			repository,
			specifier,
		}
	}
}

impl Schema for Version {
	fn sheet(&self, name: &str) -> Result<Sheet> {
		// TODO: move this stuff into the main impl

		let commit = self.repository.find_commit(self.specifier.commit)?;

		let path: PathBuf = [
			"Schemas",
			&self.specifier.game_version,
			&format!("{name}.yml"),
		]
		.iter()
		.collect();

		let entry = commit
			.tree()?
			.get_path(&path)
			.map_err(|error| match error.code() {
				ErrorCode::NotFound => Error::NotFound(ErrorValue::Other(format!("sheet {name}"))),
				_ => Error::from(error),
			})?;

		let blob = entry
			.to_object(&self.repository)?
			.into_blob()
			.map_err(|object| {
				Error::Repository(format!(
					"expected blob for {name} sheet schema, got {:?}",
					object.kind()
				))
			})?;

		parse(blob.content())
	}
}
