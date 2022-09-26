use std::path::{Path, PathBuf};

use derivative::Derivative;
use git2::{Commit, Object, Repository};
use lazy_static::lazy_static;
use serde_json::Value;

use crate::{
	error::{Error, ErrorValue, Result},
	schema::{Order, Sheet},
};

use super::parse::parse_sheet_definition;

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

	/// Get the canonical name for this version.
	pub fn canonical(&self) -> String {
		self.commit.id().to_string()
	}

	/// Get a list of all sheets supported by this version.
	pub fn sheet_names(&self) -> Result<Vec<String>> {
		// Get the tree containing sheet definitions.
		let object = self.object_at_path(&DEFINITION_PATH)?;
		let tree = object.into_tree().map_err(|object| {
			Error::Repository(format!(
				"Definition path {:?} should be a tree, got {:?}",
				*DEFINITION_PATH,
				object.kind()
			))
		})?;

		// Collect all json files in the tree.
		let iter = tree
			.iter()
			.filter_map(|entry| {
				let name = entry.name()?;
				match name.ends_with(".json") {
					true => Some(name[..name.len() - 5].to_string()),
					false => None,
				}
			})
			.collect::<Vec<_>>();

		Ok(iter)
	}

	/// Get the schema for the requested sheet at this version.
	pub fn sheet(&self, name: &str) -> Result<Sheet> {
		let path = DEFINITION_PATH.join(format!("{name}.json"));

		let object = self
			.object_at_path(&path)
			.map_err(|error| match error.code() {
				git2::ErrorCode::NotFound => {
					Error::NotFound(ErrorValue::Other(format!("Sheet {name}")))
				}
				_ => Error::from(error),
			})?;

		let blob = object.into_blob().map_err(|object| {
			Error::Repository(format!(
				"Expected blob for {} sheet schema, got {:?}",
				name,
				object.kind()
			))
		})?;

		// Trim the BOM if it's present (really, StC?)
		let mut content = blob.content();
		if content.starts_with(&[0xEF, 0xBB, 0xBF]) {
			content = &content[3..]
		}

		// TODO: handle errors better
		let value = serde_json::from_slice::<Value>(content).unwrap();
		let node = parse_sheet_definition(&value)?;

		let sheet = Sheet {
			name: name.into(),
			order: Order::Index,
			node,
		};

		Ok(sheet)
	}

	fn object_at_path(&self, path: &Path) -> Result<Object, git2::Error> {
		self.commit
			.tree()?
			.get_path(path)?
			.to_object(self.repository)
	}
}
