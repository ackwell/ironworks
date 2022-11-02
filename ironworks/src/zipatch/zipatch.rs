use std::{collections::HashMap, path::PathBuf, sync::Arc};

use super::{cache::PatchCache, version::Version};

#[derive(Debug)]
pub struct ZiPatch {
	// TODO: this should probably be arc'd. Interior or exterior mutability?
	cache: Arc<PatchCache>,
}

impl ZiPatch {
	// TODO: API. This should probably take an initial list of patches, grouped by repository or similar, and ordered by their requisite application/dependency order. alternative would be to use a self-building pattern i.e. .add/with_repository
	// i'm tempted to say Vec<String> should be a struct, instead, with "path to the patch folder" and "patches" as seperate concepts. the alternative is to accept just pathbufs anyway and encode xiv patch sorting logic; which is honestly tempting. if i do take that route, a theoreticaly api evolution could change that to `Into<Repository>` wherein a path is an autosorted repository and other options can define their own impl
	// the u8 is probably not a go on the public api honestly. if i do the builder pattern i can probably Into... and accept both, but for average-user config, passing the repository id is JANK.
	pub fn new(repositories: HashMap<u8, (PathBuf, Vec<String>)>) -> Self {
		Self {
			cache: Arc::new(PatchCache::new(repositories)),
		}
	}

	// TODO: API. Assuming going with the latter from new()'s comment, this should accept some "version" concept that declares the patch point for each repository.
	pub fn version(&self) -> Version {
		Version::new(self.cache.clone())
	}
}
