//! Schema provider backed by the SaintCoinach schema repository.

mod lcs;
mod parse;
mod provider;
mod version;

pub use {
	provider::{Provider, ProviderOptions},
	version::Version,
};
