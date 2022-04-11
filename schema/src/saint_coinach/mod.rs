//! Schema provider backed by the SaintCoinach schema repository.

mod provider;
mod version;

pub use {
	provider::{Provider, ProviderOptions},
	version::Version,
};
