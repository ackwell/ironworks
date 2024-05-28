//! Schema provider backed by the ExdSchema schema repository.

mod parse;
mod provider;
mod specifier;
mod version;

pub use {
	provider::{Provider, ProviderOptions},
	specifier::Specifier,
	version::Version,
};

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_send() {
		fn assert_send<T: Send>() {}
		assert_send::<Provider>();
		assert_send::<Version>();
	}

	#[test]
	fn test_sync() {
		fn assert_sync<T: Sync>() {}
		assert_sync::<Provider>();
		assert_sync::<Version>();
	}
}
