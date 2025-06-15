//! Modular toolkit for working with FFXIV data.

// Lint config
#![allow(clippy::module_inception)]
#![warn(missing_debug_implementations, missing_docs)]
// Doc config
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

mod filesystem;
mod utility;

#[cfg(feature = "excel")]
pub mod excel;
pub mod file;
#[cfg(feature = "sestring")]
pub mod sestring;
#[cfg(feature = "sqpack")]
pub mod sqpack;
#[cfg(feature = "zipatch")]
pub mod zipatch;

#[cfg(test)]
mod test {
	#[test]
	fn test_send() {
		fn assert_send<T: Send>() {}
	}

	#[test]
	fn test_sync() {
		fn assert_sync<T: Sync>() {}
	}
}
