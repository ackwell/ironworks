use std::{collections::HashMap, path::Path};

use anyhow::Result;

use crate::data::Data;

use super::index::Index;

pub struct Version {
	// this should probably be the canonical version struct or something?
	version: String,

	// some tables like custom/ and quest/ are going to have a name that isn't a valid file path - what do we want to use for the keys here, and should it be considered the canonical name for indices?
	indices: HashMap<String, Index>,
}

impl Version {
	pub(super) fn new(version: &str, search_path: &Path, data: &Data) -> Result<Self> {
		let path = search_path.join(version);
		// TODO: THIS SHOULD BE USING THE VERSION FROM THE ARGUMENTS. Doing so will need fixing up on the data side; which in turn will need a structured version system. should probably add that hey.
		let data_version = data.version(None);
		let excel = data_version.excel();

		// TODO: build an index for every sheet
		let sheets = ["Action"];

		let mut indices = HashMap::new();
		for name in sheets {
			// TODO: if this is async; how do i run them all at the same time?
			let index = Index::new(name, &path, excel)?;
			indices.insert(name.to_string(), index);
		}

		Ok(Self {
			version: version.to_string(),
			indices,
		})
	}

	// TODO: index specifier?
	// TODO: non-string-query filters
	pub fn search(&self, query: &str) {
		// TODO: this should combine across multiple indicies in some score-centric way?
		let x = self
			.indices
			.values()
			.map(|index| index.search(query))
			.collect::<Vec<_>>();

		tracing::debug!("search result: {x:#?}");
	}
}
