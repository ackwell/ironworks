use std::{
	collections::{hash_map::Entry, HashMap},
	hash::Hash,
	sync::{Arc, Mutex},
};

pub type HashMapCache<K, V> = Mutex<HashMap<K, Arc<V>>>;

pub trait HashMapCacheExt<K, V> {
	fn try_get_or_insert<E>(
		&self,
		key: K,
		build: impl FnOnce() -> Result<V, E>,
	) -> Result<Arc<V>, E>;
}

impl<K, V> HashMapCacheExt<K, V> for HashMapCache<K, V>
where
	K: Eq + Hash,
{
	fn try_get_or_insert<E>(
		&self,
		key: K,
		build: impl FnOnce() -> Result<V, E>,
	) -> Result<Arc<V>, E> {
		Ok(match self.lock().unwrap().entry(key) {
			Entry::Occupied(entry) => entry.get().clone(),
			Entry::Vacant(entry) => entry.insert(build()?.into()).clone(),
		})
	}
}

#[cfg(test)]
mod test {
	use std::convert::Infallible;

	use super::{HashMapCache, HashMapCacheExt};

	#[test]
	fn by_key() {
		let cache: HashMapCache<u8, u8> = Default::default();
		let mut count = 0;
		cache
			.try_get_or_insert(0, || -> Result<u8, Infallible> {
				count += 1;
				Ok(0)
			})
			.unwrap();
		cache
			.try_get_or_insert(1, || -> Result<u8, Infallible> {
				count += 1;
				Ok(1)
			})
			.unwrap();
		let value = cache
			.try_get_or_insert(1, || -> Result<u8, Infallible> {
				count += 1;
				Ok(2)
			})
			.unwrap();

		assert_eq!(*value, 1);
		assert_eq!(count, 2);
	}

	#[test]
	fn build_failures() {
		let cache: HashMapCache<u8, u8> = Default::default();
		cache.try_get_or_insert(0, || Err(())).unwrap_err();
		let value = cache
			.try_get_or_insert(1, || -> Result<u8, Infallible> { Ok(1) })
			.unwrap();
		assert_eq!(*value, 1);
	}
}
