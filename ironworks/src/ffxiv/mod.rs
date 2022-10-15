//! FFXIV specialisations and utilities for ironworks.

mod fs;
mod mapper;

pub use {
	fs::FsResource,
	mapper::{Language, Mapper},
};

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_send() {
		fn assert_send<T: Send>() {}
		assert_send::<FsResource>();
		assert_send::<Language>();
		assert_send::<Mapper>();
	}

	#[test]
	fn test_sync() {
		fn assert_sync<T: Sync>() {}
		assert_sync::<FsResource>();
		assert_sync::<Language>();
		assert_sync::<Mapper>();
	}
}
