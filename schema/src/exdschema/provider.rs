use std::{
	borrow::Cow,
	collections::HashMap,
	env::current_exe,
	path::{Path, PathBuf},
	sync::{Arc, Mutex},
};

use derivative::Derivative;
use git2::Repository;

use crate::{
	error::{Error, ErrorValue, Result},
	git::{open_repository, resolve_commit},
	schema::Sheet,
};

use super::{specifier::Specifier, version::Version};

const DEFAULT_REMOTE: &str = "https://github.com/xivdev/EXDSchema.git";
const DEFAULT_DIRECTORY: &str = "exdschema";

pub type SheetCache = Option<Arc<Mutex<HashMap<(Specifier, String), Result<Sheet>>>>>;

/// Schema provider sourcing data from the xivdev EXDSchema repository.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Provider {
	#[derivative(Debug = "ignore")]
	pub(super) repository: Arc<Mutex<Repository>>,

	cache: SheetCache,
}

impl Provider {
	/// Construct a `Provider` with default configuration.
	pub fn new() -> Result<Self> {
		Self::with().build()
	}

	/// Create an options object to configure a `Provider` with.
	pub fn with() -> ProviderOptions {
		ProviderOptions::default()
	}

	fn with_options(options: ProviderOptions) -> Result<Self> {
		let remote = options.remote.as_deref().unwrap_or(DEFAULT_REMOTE);

		let directory = options
			.directory
			.as_ref()
			.map(Cow::from)
			.or_else(default_directory)
			.ok_or_else(|| Error::NotFound(ErrorValue::Other("repository directory".into())))?;

		let repository = open_repository(remote, &directory)?;

		Ok(Self {
			repository: Arc::new(Mutex::new(repository)),
			cache: options.cache.then(|| Default::default()),
		})
	}

	/// Attempt to fetch updates to the schema repository. An error will only be
	/// returned if the fetch could not be performed. Boolean value on success
	/// represents whether a change to the repository's HEAD was observed as a
	/// result of the update.
	pub fn update(&self) -> Result<bool> {
		let repository = self.repository.lock().unwrap();

		// Get the current HEAD commit oid.
		let old_head = resolve_commit(&repository, "HEAD")?.id();

		// Fetch + update the origin.
		repository
			.find_remote("origin")?
			.fetch::<&str>(&[], None, None)?;

		// Check if we've got a new HEAD.
		let new_head = resolve_commit(&repository, "HEAD")?.id();

		Ok(new_head != old_head)
	}

	/// Fetch a `Specifier` that matches the specified git reference and game version as closely as possible.
	pub fn specifier(&self, reference: &str, game_version: &str) -> Result<Specifier> {
		Specifier::new(self, reference, game_version)
	}

	/// Fetch the specified version of the schema.
	pub fn version(&self, specifier: Specifier) -> Result<Version> {
		Ok(Version::new(
			Arc::clone(&self.repository),
			self.cache.clone(),
			specifier,
		))
	}
}

/// Configuration builder for EXDSchema provider.
#[derive(Debug)]
pub struct ProviderOptions {
	remote: Option<String>,
	directory: Option<PathBuf>,
	cache: bool,
}

impl ProviderOptions {
	fn new() -> Self {
		Self {
			remote: None,
			directory: None,
			cache: false,
		}
	}

	/// Set the git remote to fetch the EXDSchema repository from.
	pub fn remote(mut self, remote: impl Into<String>) -> Self {
		self.remote = Some(remote.into());
		self
	}

	/// Set the local directory to clone SaintCoinach into.
	pub fn directory(mut self, directory: impl Into<PathBuf>) -> Self {
		self.directory = Some(directory.into());
		self
	}

	/// Enable or disable the caching of sheet schemas.
	pub fn cache(mut self, cache: bool) -> Self {
		self.cache = cache;
		self
	}

	/// Construct a `Provider` instance with the given configuration.
	pub fn build(self) -> Result<Provider> {
		Provider::with_options(self)
	}
}

impl Default for ProviderOptions {
	fn default() -> Self {
		Self::new()
	}
}

fn default_directory<'a>() -> Option<Cow<'a, Path>> {
	let path = current_exe().ok()?.parent()?.join(DEFAULT_DIRECTORY);
	Some(path.into())
}
