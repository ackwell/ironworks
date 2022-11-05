use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use crate::error::Result;

use super::{lookup::PatchLookup, repository::PatchRepository, version::Version};

#[derive(Debug)]
pub struct ZiPatch {
	repositories: HashMap<u8, Arc<PatchRepository>>,

	data: Arc<LookupCache>,
}

impl ZiPatch {
	pub fn new() -> Self {
		Self {
			repositories: HashMap::default(),
			data: Arc::new(LookupCache::new()),
		}
	}

	pub fn with_repository(mut self, id: u8, repository: PatchRepository) -> Self {
		self.add_repository(id, repository);
		self
	}

	pub fn add_repository(&mut self, id: u8, repository: PatchRepository) {
		self.repositories.insert(id, Arc::new(repository));
	}

	// TODO: API. Assuming going with the latter from new()'s comment, this should accept some "version" concept that declares the patch point for each repository.
	pub fn version(&self) -> Version {
		// note; snapshotting repositories state here is intentional. doc it.
		Version::new(self.repositories.clone(), self.data.clone())
	}
}

#[derive(Debug)]
pub struct LookupCache {
	cache: RwLock<HashMap<(u8, String), Arc<PatchLookup>>>,
}

impl LookupCache {
	pub fn new() -> Self {
		Self {
			cache: Default::default(),
		}
	}

	// TODO: Not a fan of both repo id and repo in this sig. Consider how that can be improved.
	pub fn lookup(
		&self,
		repository_id: u8,
		repository: &PatchRepository,
		patch: &str,
	) -> Result<Arc<PatchLookup>> {
		// TODO: Can I avoid the clone on the string? Seems shit.
		let key = (repository_id, patch.to_string());

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
	}
}
