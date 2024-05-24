//! Schema provider backed by the ExdSchema schema repository.

mod provider;
mod specifier;
mod version;

pub use {
	provider::{Provider, ProviderOptions},
	specifier::{IntoSpecifier, Specifier},
};
