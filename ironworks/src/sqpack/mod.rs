//! Tools for working with the SqPack package format.

mod file;
mod index;
mod resource;
mod sqpack;

pub use {file::File, resource::Resource, sqpack::SqPack};

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_send() {
		fn assert_send<T: Send>() {}
		assert_send::<File<()>>();
		assert_send::<SqPack<()>>();
	}

	#[test]
	fn test_sync() {
		fn assert_sync<T: Sync>() {}
		assert_sync::<File<()>>();
		assert_sync::<SqPack<()>>();
	}
}
