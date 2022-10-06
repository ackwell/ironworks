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

	// TODO: who "owns" data ref? - i don't think search needs data outside the init step?
	// is there any point in this being seperate from new(), really?
	pub fn initialize(&mut self, data: &Data) -> Result<()> {
		// ... do... i want to pass this shit to version and let it pin down, or do i want to pin down here and pass shit down to version? like i guess if anything the index needs the sheet name so it can lazy init an excel sheet for ingest so keeping that up for the other shit makes sense?
		let version = Version::new("TODO VERSION", &self.path, data)?;

		// TODO: I'm tempted to say that indexing versions should be lazy but... idk. check how long it takes to index a full gamever - if it's a notable duration on my computer it'll probably be glacial on a server.
		self.temp_version = Some(Arc::new(version));

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
	}
}
