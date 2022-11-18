use std::{env::current_exe, path::PathBuf, sync::Arc};

use anyhow::Result;

use crate::data::Data;

use super::version::Version;

pub struct Search {
	path: PathBuf,

	// TODO: this should be a map of version keys to search::Version instances
	temp_version: Option<Arc<Version>>,
}

impl Search {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		// TODO: configurable directory, this shouldn't be touching current exe at all
		let path = current_exe()
			.expect("could not resolve current executable")
			.parent()
			.expect("current path has no parent")
			.join("search");

		Self {
			path,
			temp_version: None,
		}
	}

	// TODO: name. ensure_ingested()? is it worth naming it to flag that ingestion may not occur?
	pub fn ingest(&mut self, data: &Data, version: Option<&str>) -> Result<()> {
		let data_version = data.version(version);
		let mut search_version = Version::new(self.path.join(version.unwrap_or("__NONE")));

		search_version.ingest(data_version)?;

		self.temp_version = Some(Arc::new(search_version));

		Ok(())
	}

	pub fn version(&self, version: Option<&str>) -> Arc<Version> {
		// TODO: actual version handling
		if version.is_some() {
			todo!("search version handling");
		}

		self.temp_version
			.clone()
			.expect("todo: how do i handle search not being instantiated?")
		// ^ probably return Option<T> from this function, and let the http side return a "this version is not searchable" error or something
		// ... ingestion will likely take Some Time:tm: per version - either ingest() should only add to the map when it's complete, or alternatively versions will need to mark when they've finished ingesting so this can avoid returning incomplete ones.
	}
}
