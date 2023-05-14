use std::{
	collections::{hash_map::DefaultHasher, HashMap},
	hash::BuildHasherDefault,
	sync::Arc,
	time::Duration,
};

use mini_moka::sync::Cache;
use uuid::Uuid;

use crate::{search::internal_query::post, version::VersionKey};

use super::provider::{IndexKey, SheetKey};

pub struct Cursor {
	pub version: VersionKey,
	pub indices: StableHashMap<IndexKey, IndexCursor>,
}

pub type StableHashMap<K, V> = HashMap<K, V, BuildHasherDefault<DefaultHasher>>;

#[derive(Default)]
pub struct IndexCursor {
	pub queries: Vec<(SheetKey, post::Node)>,
	pub offset: usize,
}

pub struct CursorCache {
	cache: Cache<Uuid, Arc<Cursor>>,
}

impl CursorCache {
	pub fn new() -> Self {
		// TODO: configuration for this
		let cache = Cache::builder()
			.time_to_idle(Duration::from_secs(5 * 60))
			.build();
		Self { cache }
	}

	pub fn get(&self, key: Uuid) -> Option<Arc<Cursor>> {
		self.cache.get(&key)
	}

	pub fn insert(&self, cursor: Cursor) -> Uuid {
		let key = Uuid::new_v4();
		self.cache.insert(key, Arc::new(cursor));
		key
	}
}
