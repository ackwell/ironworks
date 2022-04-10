use std::{
	borrow::Cow,
	env::current_exe,
	path::{Path, PathBuf},
};

use crate::error::{Error, ErrorValue, Result};

// Default configuration
const REPOSITORY_URL: &str = "https://github.com/xivapi/SaintCoinach.git";
const REPOSITORY_DIRECTORY: &str = "saint_coinach";

/// Configuration option builder for the Saint Coinach schema provider.
#[derive(Debug)]
pub struct ProviderOptions {
	remote: Option<String>,
	directory: Option<PathBuf>,
}

impl ProviderOptions {
	fn new() -> Self {
		ProviderOptions {
			remote: None,
			directory: None,
		}
	}

	/// Set the git remote URL to fetch SaintCoinach from.
	pub fn remote(&mut self, remote: impl ToString) -> &mut Self {
		self.remote = Some(remote.to_string());
		self
	}

	/// Set the local directory to clone SaintCoinach to.
	pub fn directory(&mut self, directory: impl Into<PathBuf>) -> &mut Self {
		self.directory = Some(directory.into());
		self
	}

	/// Build a `Provider` instance with the given configuration.
	pub fn build(&self) -> Result<Provider> {
		Provider::with_options(self)
	}
}

impl Default for ProviderOptions {
	fn default() -> Self {
		Self::new()
	}
}

/// A schema provider sourcing data from the SaintCoinach schema repository.
#[derive(Debug)]
pub struct Provider {}

impl Provider {
	/// Build a new `Provider` with default configuration.
	pub fn new() -> Result<Self> {
		Self::with().build()
	}

	/// Build a `Provider` with additional configuration.
	pub fn with() -> ProviderOptions {
		Default::default()
	}

	fn with_options(options: &ProviderOptions) -> Result<Self> {
		let remote = options.remote.as_deref().unwrap_or(REPOSITORY_URL);

		let directory = options
			.directory
			.as_ref()
			.map(Cow::from)
			.or_else(default_directory)
			.ok_or_else(|| Error::NotFound(ErrorValue::Other("Repository directory".into())))?;

		Ok(Self {})
	}
}

// TODO: Maybe... lazy static this? Would that work?
fn default_directory<'a>() -> Option<Cow<'a, Path>> {
	current_exe().ok().and_then(|path| {
		path.parent()
			.map(|parent| parent.join(REPOSITORY_DIRECTORY).into())
	})
}
