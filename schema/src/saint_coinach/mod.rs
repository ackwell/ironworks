//! Schema provider backed by the SaintCoinach schema repository.

mod lcs;
mod parse;
mod provider;
mod version;

pub use {
	provider::{Provider, ProviderOptions},
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
