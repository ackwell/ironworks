//! FFXIV specialisations and utilities for ironworks.

mod fs;

pub use fs::FsResource;

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_send() {
		fn assert_send<T: Send>() {}
		assert_send::<FsResource>();
	}

	#[test]
	fn test_sync() {
		fn assert_sync<T: Sync>() {}
		assert_sync::<FsResource>();
	}
}
