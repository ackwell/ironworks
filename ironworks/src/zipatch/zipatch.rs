use std::{
	collections::{hash_map::Entry, HashMap},
	path::PathBuf,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc, Condvar, Mutex,
	},
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

	/// Enable persistance of lookup tables used when reading patch files. Enabling
	/// this will cause additional files to be written alongside patch files.
	pub fn with_persisted_lookups(mut self) -> Self {
		self.persist_lookups();
		self
	}

	/// Enable persistance of lookup tables used when reading patch files. Enabling
	/// this will cause additional files to be written alongside patch files.
	pub fn persist_lookups(&mut self) {
		self.cache.persist_lookups()
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
	persist_lookups: AtomicBool,
	cache: Mutex<HashMap<PathBuf, CacheSync<Arc<PatchLookup>>>>,
}

impl LookupCache {
	pub fn new() -> Self {
		Self {
			persist_lookups: false.into(),
			cache: Default::default(),
		}
	}

	fn persist_lookups(&self) {
		self.persist_lookups.store(true, Ordering::SeqCst)
	}

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
		let lookup = Arc::new(self.read_lookup(patch)?);

		// Write the new lookup to the cache.
		let mut value = mutex.lock().unwrap();
		*value = Some(lookup.clone());
		condvar.notify_all();

		Ok(lookup)
	}

	fn read_lookup(&self, patch: &Patch) -> Result<PatchLookup> {
		let persist_lookups = self.persist_lookups.load(Ordering::SeqCst);
		if !persist_lookups {
			return PatchLookup::build(&patch.path);
		}

		let mut lut_path = patch.path.as_os_str().to_owned();
		lut_path.push(".lut");
		let lut_path = PathBuf::from(lut_path);

		PatchLookup::from_cache(&patch.path, &lut_path)
	}
}
