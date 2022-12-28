use std::{
	path::{Path, PathBuf},
	sync::{Arc, Mutex},
};

use derivative::Derivative;
use git2::{Object, Oid, Repository};
use lazy_static::lazy_static;
use serde_json::Value;

use crate::{
	error::{Error, ErrorValue, Result},
	schema::{Order, Sheet},
	Schema,
};

use super::parse::parse_sheet_definition;

lazy_static! {
	static ref DEFINITION_PATH: PathBuf = ["SaintCoinach", "Definitions"].iter().collect();
}

/// A single version of the SaintCoinach schema.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Version {
	#[derivative(Debug = "ignore")]
	repository: Arc<Mutex<Repository>>,

	// TODO: Consider storing the OID for the Definitions directory tree object rather than the top-level commit object.
	commit_id: Oid,
}

impl Version {
	pub(super) fn new(repository: Arc<Mutex<Repository>>, commit_id: Oid) -> Self {
		Version {
			repository,
			commit_id,
		}
	}

	/// Get the canonical name for this version.
	pub fn canonical(&self) -> String {
		self.commit_id.to_string()
	}

	/// Get a list of all sheets supported by this version.
	pub fn sheet_names(&self) -> Result<Vec<String>> {
		let repository = self.repository.lock().unwrap();

		// Get the tree containing sheet definitions.
		let object = self.object_at_path(&repository, &DEFINITION_PATH)?;
		let tree = object.into_tree().map_err(|object| {
			Error::Repository(format!(
				"Definition path {:?} should be a tree, got {:?}",
				*DEFINITION_PATH,
				object.kind()
			))
		})?;

		// Collect all json files in the tree.
		let sheet_names = tree
			.iter()
			.filter_map(|entry| {
				let name = entry.name()?;
				match name.ends_with(".json") {
					true => Some(name[..name.len() - 5].to_string()),
					false => None,
				}
			})
			.collect::<Vec<_>>();

		Ok(sheet_names)
	}

	fn object_at_path<'a>(
		&self,
		repository: &'a Repository,
		path: &Path,
	) -> Result<Object<'a>, git2::Error> {
		repository
			.find_commit(self.commit_id)?
			.tree()?
			.get_path(path)?
			.to_object(repository)
	}
}

impl Schema for Version {
	fn sheet(&self, name: &str) -> Result<Sheet> {
		// TODO: This currently locks the repository for all consumers until it has completed parsing, with the benefit of not copying the blob data into memory before running the parse. If the potential contention on this proves problematic, pull blob data into memory and drop the guard early.
		let repository = self.repository.lock().unwrap();
		let path = DEFINITION_PATH.join(format!("{name}.json"));

		let object =
			self.object_at_path(&repository, &path)
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
}
