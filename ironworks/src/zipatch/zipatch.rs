use std::{
	collections::{hash_map::Entry, HashMap},
	sync::{Arc, Condvar, Mutex},
};

use crate::error::Result;

use super::{
	lookup::PatchLookup,
	repository::{Patch, PatchRepository},
	version::{Version, VersionSpecifier},
};

/// A struct providing access to data contained in ZiPatch-formatted patch files.
#[derive(Debug)]
pub struct ZiPatch {
	repositories: HashMap<u8, Arc<PatchRepository>>,

	data: Arc<LookupCache>,
}

impl ZiPatch {
	/// Create a blank ZiPatch instance.
	pub fn new() -> Self {
		Self {
			repositories: HashMap::default(),
			data: Arc::new(LookupCache::new()),
		}
	}

	/// Add a patch repository for the given SqPack repository ID.
	pub fn with_repository(mut self, id: u8, repository: PatchRepository) -> Self {
		self.add_repository(id, repository);
		self
	}

	/// Add a patch repository for the given SqPack repository ID.
	pub fn add_repository(&mut self, id: u8, repository: PatchRepository) {
		self.repositories.insert(id, Arc::new(repository));
	}

	/// Create a view into the patch data at the specified version. Repositories
	/// configured with this instance will be snapshot at the point in time the
	/// version is created.
	pub fn version(&self, specifier: VersionSpecifier) -> Version {
		// note; snapshotting repositories state here is intentional. doc it.
		Version::new(specifier, self.repositories.clone(), self.data.clone())
	}
}

impl Default for ZiPatch {
	fn default() -> Self {
		Self::new()
	}
}

type CacheKey = (u8, String); // (repository, patch_name)
type CacheSync<T> = Arc<(Mutex<Option<T>>, Condvar)>;

#[derive(Debug)]
pub struct LookupCache {
	cache: Mutex<HashMap<CacheKey, CacheSync<Arc<PatchLookup>>>>,
}

impl LookupCache {
	pub fn new() -> Self {
		Self {
			cache: Default::default(),
		}
	}

	pub fn lookup(&self, repository_id: u8, patch: &Patch) -> Result<Arc<PatchLookup>> {
		// TODO: Can I avoid the clone on the string? Seems shit.
		let key = (repository_id, patch.name.clone());

		// TODO: honestly this might make sense as an alternate impl of the hashmapcache
		// Get a lock on the main cache and fetch the internal sync primative. We're
		// also recording if it existed prior to this call.
		let mut cache = self.cache.lock().unwrap();
		let (occupied, value) = match cache.entry(key) {
			Entry::Occupied(entry) => (true, entry.get().clone()),
			Entry::Vacant(entry) => (
				false,
				entry
					.insert(Arc::new((Mutex::new(None), Condvar::new())))
					.clone(),
			),
		};
		drop(cache);

		let (mutex, condvar) = &*value;

		// If the cache entry already existed, some other thread is building the
		// lookup already - wait for it to complete via the condvar.
		if occupied {
			let mut value = mutex.lock().unwrap();
			while value.is_none() {
				value = condvar.wait(value).unwrap();
			}
			return Ok(value.as_ref().expect("lock condition broken").clone());
		}

		// Build a new lookup for this patch.
		let lookup = Arc::new(PatchLookup::new(&patch.path)?);

		// Write the new lookup to the cache.
		let mut value = mutex.lock().unwrap();
		*value = Some(lookup.clone());
		condvar.notify_all();

		Ok(lookup)
	}
}
