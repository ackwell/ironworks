//! Tools for working with the SqPack package format.

mod block;
mod file;
mod index;
mod install;
mod resource;
mod sqpack;
mod vinstall;

pub use {
	block::{BlockMetadata, BlockPayload, BlockStream},
	file::File,
	index::Location,
	install::Install,
	resource::Resource,
	sqpack::SqPack,
	vinstall::{VInstall, Vfs},
};

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
