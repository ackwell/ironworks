//! Tools for working with the Excel database format.

mod error;
mod excel;
mod language;
mod path;

pub use {excel::Excel, language::Language};

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_send() {
		fn assert_send<T: Send>() {}
	}

	#[test]
	fn test_sync() {
		fn assert_sync<T: Sync>() {}
	}
}
