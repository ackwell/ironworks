use std::{
	collections::{hash_map::DefaultHasher, HashMap},
	hash::BuildHasherDefault,
	sync::Arc,
	time::Duration,
};

use mini_moka::sync as moka;
use serde::Deserialize;
use uuid::Uuid;

use crate::{search::internal_query::post, version::VersionKey};

use super::key::{IndexKey, SheetKey};

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

#[derive(Debug, Deserialize)]
pub struct Config {
	ttl: Option<u64>,
	tti: Option<u64>,
}

pub struct Cache {
	cache: moka::Cache<Uuid, Arc<Cursor>>,
}

impl Cache {
	pub fn new(config: Config) -> Self {
		let mut builder = moka::Cache::builder();
		if let Some(ttl) = config.ttl {
			builder = builder.time_to_live(Duration::from_secs(ttl));
		}
		if let Some(tti) = config.tti {
			builder = builder.time_to_idle(Duration::from_secs(tti));
		}

		Self {
			cache: builder.build(),
		}
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
