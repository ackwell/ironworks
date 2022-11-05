use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use crate::error::{Error, ErrorValue, Result};

use super::{lookup::PatchLookup, repository::PatchRepository, version::Version};

#[derive(Debug)]
pub struct ZiPatch {
	// TODO: this should probably be arc'd. Interior or exterior mutability?
	data: Arc<ZiPatchData>,
}

impl ZiPatch {
	// TODO: API. This should probably take an initial list of patches, grouped by repository or similar, and ordered by their requisite application/dependency order. alternative would be to use a self-building pattern i.e. .add/with_repository
	// i'm tempted to say Vec<String> should be a struct, instead, with "path to the patch folder" and "patches" as seperate concepts. the alternative is to accept just pathbufs anyway and encode xiv patch sorting logic; which is honestly tempting. if i do take that route, a theoreticaly api evolution could change that to `Into<Repository>` wherein a path is an autosorted repository and other options can define their own impl
	// the u8 is probably not a go on the public api honestly. if i do the builder pattern i can probably Into... and accept both, but for average-user config, passing the repository id is JANK.
	pub fn new(repositories: HashMap<u8, PatchRepository>) -> Self {
		Self {
			data: Arc::new(ZiPatchData::new(repositories)),
		}
	}

	// TODO: API. Assuming going with the latter from new()'s comment, this should accept some "version" concept that declares the patch point for each repository.
	pub fn version(&self) -> Version {
		Version::new(self.data.clone())
	}
}

#[derive(Debug)]
pub struct ZiPatchData {
	repositories: HashMap<u8, PatchRepository>,
	cache: RwLock<HashMap<(u8, String), Arc<PatchLookup>>>,
}

impl ZiPatchData {
	pub fn new(repositories: HashMap<u8, PatchRepository>) -> Self {
		Self {
			repositories,
			cache: Default::default(),
		}
	}

	// TODO: flatten that outer result maybe?
	pub fn patch_lookups(
		&self,
		repository_id: u8,
		// TODO: this needs a version param to skip meta prior to.
	) -> Result<impl Iterator<Item = Result<Arc<PatchLookup>>> + '_> {
		let repository = self.repositories.get(&repository_id).ok_or_else(|| {
			Error::NotFound(ErrorValue::Other(format!("repository {repository_id}")))
		})?;

		// We're operating at a patch-by-patch granularity here, with the (very safe)
		// assumption that a game version is at minimum one patch.
		let iterator = repository.patches.iter().rev().map(move |patch| {
			// TODO: Can I avoid the clone on the string? Seems shit.
			let key = (repository_id, patch.clone());

			// TODO: honestly this might make sense as an alternate impl of the hashmapcache
			// Grab a read guard and try to get an existing lookup.
			let cache_read = self.cache.read().unwrap();
			if let Some(lookup) = cache_read.get(&key) {
				return Ok(Arc::clone(lookup));
			};
			drop(cache_read);

			// Build a new lookup for this patch.
			let lookup = Arc::new(PatchLookup::new(
				&repository.base_directory.join(format!("{patch}.patch")),
			)?);

			// Write the new lookup to the cache.
			let mut cache_write = self.cache.write().unwrap();
			let lookup = cache_write.entry(key).or_insert(lookup);
			Ok(Arc::clone(lookup))
		});
		Ok(iterator)
	}
}
