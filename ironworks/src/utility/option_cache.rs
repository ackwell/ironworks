use std::{cell::RefCell, rc::Rc};

pub type OptionCache<T> = RefCell<Option<Rc<T>>>;

pub trait OptionCacheExt<T> {
	fn try_get_or_insert<F, E>(&self, build: F) -> Result<Rc<T>, E>
	where
		F: FnOnce() -> Result<T, E>;
}

impl<T> OptionCacheExt<T> for OptionCache<T> {
	fn try_get_or_insert<F, E>(&self, build: F) -> Result<Rc<T>, E>
	where
		F: FnOnce() -> Result<T, E>,
	{
		Ok(match &mut *self.borrow_mut() {
			Some(inner) => inner.clone(),
			option @ None => option.insert(build()?.into()).clone(),
		})
	}
}

#[cfg(test)]
mod test {
	use std::convert::Infallible;

	use super::{OptionCache, OptionCacheExt};

	#[test]
	fn default() {
		let cache: OptionCache<u8> = Default::default();
		assert_eq!(cache.into_inner(), None)
	}

	#[test]
	fn builds_once() {
		let cache: OptionCache<u8> = Default::default();
		let mut count = 0;
		cache
			.try_get_or_insert(|| -> Result<u8, Infallible> {
				count += 1;
				Ok(1)
			})
			.unwrap();
		let value = cache
			.try_get_or_insert(|| -> Result<u8, Infallible> {
				count += 1;
				Ok(2)
			})
			.unwrap();

		assert_eq!(*value, 1);
		assert_eq!(count, 1);
	}

	#[test]
	fn build_failures() {
		let cache: OptionCache<u8> = Default::default();
		cache.try_get_or_insert(|| Err(())).unwrap_err();
		let value = cache
			.try_get_or_insert(|| -> Result<u8, Infallible> { Ok(1) })
			.unwrap();
		assert_eq!(*value, 1);
	}
}
