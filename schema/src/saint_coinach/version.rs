use std::path::{Path, PathBuf};

use derivative::Derivative;
use git2::{Commit, Object, Repository};
use lazy_static::lazy_static;

use crate::error::{Error, ErrorValue, Result};

lazy_static! {
	static ref DEFINITION_PATH: PathBuf = ["SaintCoinach", "Definitions"].iter().collect();
}

/// A single version of the SaintCoinach schema.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Version<'repo> {
	#[derivative(Debug = "ignore")]
	repository: &'repo Repository,
	commit: Commit<'repo>,
}

impl<'repo> Version<'repo> {
	pub(super) fn new(repository: &'repo Repository, commit: Commit<'repo>) -> Self {
		Version { repository, commit }
	}

	/// Get the schema for the requested sheet at this version.
	pub fn schema(&self, sheet: &str) -> Result<()> {
		let path = DEFINITION_PATH.join(format!("{sheet}.json"));

		let object = self
			.object_at_path(&path)
			.map_err(|error| match error.code() {
				git2::ErrorCode::NotFound => {
					Error::NotFound(ErrorValue::Other(format!("Sheet {sheet}")))
				}
				_ => Error::from(error),
			})?;

		let blob = object.as_blob().ok_or_else(|| {
			Error::Repository(format!(
				"Expected blob for {} sheet schema, got {:?}",
				sheet,
				object.kind()
			))
		})?;

		println!("{}", String::from_utf8_lossy(blob.content()));

		Ok(())
	}

	fn object_at_path(&self, path: &Path) -> Result<Object, git2::Error> {
		self.commit
			.tree()?
			.get_path(path)?
			.to_object(self.repository)
	}
}
