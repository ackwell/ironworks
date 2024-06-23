use std::{
	borrow::Cow,
	collections::HashMap,
	env::current_exe,
	path::{Path, PathBuf},
	sync::{Arc, Mutex},
};

use derivative::Derivative;
use git2::{Oid, Repository};

use crate::{
	error::{Error, ErrorValue, Result},
	git::{open_repository, resolve_commit},
	Sheet,
};

use super::version::Version;

// Default configuration
const REPOSITORY_URL: &str = "https://github.com/xivapi/SaintCoinach.git";
const REPOSITORY_DIRECTORY: &str = "saint_coinach";

/// Configuration option builder for the Saint Coinach schema provider.
#[derive(Debug)]
pub struct ProviderOptions {
	remote: Option<String>,
	directory: Option<PathBuf>,
	cache: bool,
}

impl ProviderOptions {
	fn new() -> Self {
		ProviderOptions {
			remote: None,
			directory: None,
			cache: true,
		}
	}

	/// Set the git remote URL to fetch SaintCoinach from.
	pub fn remote(mut self, remote: impl ToString) -> Self {
		self.remote = Some(remote.to_string());
		self
	}

	/// Set the local directory to clone SaintCoinach to.
	pub fn directory(mut self, directory: impl Into<PathBuf>) -> Self {
		self.directory = Some(directory.into());
		self
	}

	/// Enable or disable caching of sheet schemas.
	pub fn cache(mut self, cache: bool) -> Self {
		self.cache = cache;
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

// TODO: per notes; look into allowing support for multiple readers without race conditions
pub type SheetCache = Option<Arc<Mutex<HashMap<(Oid, String), Result<Sheet>>>>>;

/// A schema provider sourcing data from the SaintCoinach schema repository.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Provider {
	#[derivative(Debug = "ignore")]
	repository: Arc<Mutex<Repository>>,

	cache: SheetCache,
}

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
			.ok_or_else(|| Error::NotFound(ErrorValue::Other("repository directory".into())))?;

		let repository = open_repository(remote, &directory)?;

		Ok(Self {
			repository: Arc::new(Mutex::new(repository)),
			cache: options.cache.then(|| Arc::new(Mutex::new(HashMap::new()))),
		})
	}

	/// Fetch the specified version of the schema.
	pub fn version(&self, version: &str) -> Result<Version> {
		let repository = self.repository.lock().unwrap();

		let commit = resolve_commit(&repository, version)?;

		Ok(Version::new(
			self.repository.clone(),
			self.cache.clone(),
			commit.id(),
		))
	}
}

fn default_directory<'a>() -> Option<Cow<'a, Path>> {
	let path = current_exe().ok()?.parent()?.join(REPOSITORY_DIRECTORY);
	Some(path.into())
}
