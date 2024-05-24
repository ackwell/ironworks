//! Schema provider backed by the ExdSchema schema repository.

mod provider;
mod specifier;

pub use {
	provider::{Provider, ProviderOptions},
	specifier::{IntoSpecifier, Specifier},
};
