use std::{
	fmt::Display,
	hash::Hash,
	path::{Path, PathBuf},
};

use git2::{Commit, Object, Repository};
use ironworks_schema_core::Node;
use lazy_static::lazy_static;
use serde_json::Value;

use crate::{
	error::{Error, Result},
	parse::parse_sheet_definition,
};

lazy_static! {
	static ref DEFINITION_PATH: PathBuf = ["SaintCoinach", "Definitions"].iter().collect();
}

// this should impl a "version" trait or something
pub struct SaintCoinachVersion<'repo> {
	// Should we be Rc-ing the repo so versions can live seperately? Not sure how the lifetime on the commit would work there.
	repository: &'repo Repository,
	commit: Commit<'repo>,
}

impl<'repo> SaintCoinachVersion<'repo> {
	pub(crate) fn new(repository: &'repo Repository, commit: Commit<'repo>) -> Self {
		Self { repository, commit }
	}

	// thoughts; for hash map keying & stuff
	pub fn id(&self) -> impl Eq + Hash + Display {
		self.commit.id()
	}

	// fn schemas -> iter

	pub fn schema(&self, sheet: &str) -> Result<Node> {
		let path = DEFINITION_PATH.join(format!("{}.json", sheet));

		let object = self
			.object_at_path(&path)
			.map_err(|error| match error.code() {
				git2::ErrorCode::NotFound => {
					Error::NotFound(format!("Definition for sheet {}", sheet))
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

		let value = serde_json::from_slice::<Value>(blob.content())
			.map_err(|error| Error::Schema(format!("Failed to parse schema: {}", error)))?;
		let schema = parse_sheet_definition(&value)?;

		Ok(schema)
	}

	fn object_at_path(&self, path: &Path) -> Result<Object<'_>, git2::Error> {
		self.commit
			.tree()?
			.get_path(path)?
			.to_object(self.repository)
	}
}
