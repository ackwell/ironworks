use std::{
	collections::{hash_map::Entry, HashMap},
	path::PathBuf,
	sync::{Arc, Condvar, Mutex},
};

use crate::error::Result;

use super::{lookup::PatchLookup, repository::Patch, view::ViewBuilder};

/// A struct providing access to data contained in ZiPatch-formatted patch files.
#[derive(Debug)]
pub struct ZiPatch {
	cache: Arc<LookupCache>,
}

impl ZiPatch {
	/// Create a blank ZiPatch instance.
	pub fn new() -> Self {
		Self {
			cache: Arc::new(LookupCache::new()),
		}
	}

	/// Build a view of patch repository files to be used as a SqPack resource.
	pub fn view(&self) -> ViewBuilder {
		ViewBuilder::new(self.cache.clone())
	}
}

impl Default for ZiPatch {
	fn default() -> Self {
		Self::new()
	}
}

type CacheSync<T> = Arc<(Mutex<Option<T>>, Condvar)>;

#[derive(Debug)]
pub struct LookupCache {
	cache: Mutex<HashMap<PathBuf, CacheSync<Arc<PatchLookup>>>>,
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

	pub fn lookup(&self, patch: &Patch) -> Result<Arc<PatchLookup>> {
		// TODO: honestly this might make sense as an alternate impl of the hashmapcache
		// Get a lock on the main cache and fetch the internal sync primative. We're
		// also recording if it existed prior to this call.
		let mut cache = self.cache.lock().unwrap();
		let (occupied, value) = match cache.entry(patch.path.clone()) {
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
