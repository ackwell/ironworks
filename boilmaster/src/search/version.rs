use std::{
	collections::{hash_map::Entry, HashMap},
	path::PathBuf,
};

use anyhow::Result;

use crate::data::Version as DataVersion;

use super::index::Index;

pub struct Version {
	path: PathBuf,

	indices: HashMap<String, Index>,
}

impl Version {
	pub(super) fn new(path: PathBuf) -> Self {
		Self {
			path,
			indices: Default::default(),
		}
	}

	pub(super) async fn ingest(&mut self, data: &DataVersion) -> Result<()> {
		let excel = data.excel();

		// NOTE: should probably record which sheets contain strings so we can immediately ignore the rest when there's a query string

		// TODO: on zipatch-backed data instances, accessing .list() could block for quite some time - how do i want to handle that?
		for sheet_name in excel.list()?.iter() {
			match self.indices.entry(sheet_name.to_string()) {
				Entry::Occupied(entry) => entry.into_mut(),
				Entry::Vacant(entry) => entry.insert(
					Index::ingest(
						&self.path.join(sheet_name.replace('/', "!DIR!")),
						excel.sheet(sheet_name.to_string())?,
					)
					.await?,
				),
			};
		}

		Ok(())
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
