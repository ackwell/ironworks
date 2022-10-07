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

		// NOTE: should probably record which sheets contain strings so we can immediately ignore the rest when there's a query string
		let mut indices = HashMap::new();
		for name in excel.list()?.iter() {
			// TODO: if this is async; how do i run them all at the same time?
			let index = Index::new(name.as_ref(), &path, excel)?;
			indices.insert(name.to_string(), index);
		}

		Ok(Self {
			version: version.to_string(),
			indices,
		})
	}

	// TODO: index specifier?
	// TODO: non-string-query filters
	// TODO: continuation?
	pub fn search(&self, query: &str) -> Result<impl Iterator<Item = (f32, (&str, u32, u16))>> {
		// Get an iterator for each of the indexes, lifting any errors from the initial search execution.
		// TODO: this needs to tag results with their index in some manner because at the moment it immeidatly gets lost
		let index_results = self
			.indices
			.iter()
			.map(|(name, index)| {
				let tagged_results = index
					.search(query)?
					.map(|(score, (row, subrow))| (score, (name.as_str(), row, subrow)));
				Ok(tagged_results)
			})
			.collect::<Result<Vec<_>>>()?;

		let iterator = index_results.into_iter().flatten();

		Ok(iterator)
	}
}
