use std::{
	borrow::Cow,
	env::current_exe,
	path::{Path, PathBuf},
	sync::Arc,
};

use derivative::Derivative;
use git2::Repository;

use crate::{
	error::{Error, ErrorValue, Result},
	git::{open_repository, resolve_commit},
};

use super::{specifier::Specifier, version::Version, TryIntoSpecifier};

const DEFAULT_REMOTE: &str = "https://github.com/xivdev/EXDSchema.git";
const DEFAULT_DIRECTORY: &str = "exdschema";

/// Schema provider sourcing data from the xivdev EXDSchema repository.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Provider {
	#[derivative(Debug = "ignore")]
	pub(super) repository: Arc<Repository>,
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
			repository: Arc::new(repository),
		})
	}

	/// Attempt to fetch updates to the schema repository. An error will only be
	/// returned if the fetch could not be performed. Boolean value on success
	/// represents whether a change to the repository's HEAD was observed as a
	/// result of the update.
	pub fn update(&self) -> Result<bool> {
		// Get the current HEAD commit oid.
		let old_head = resolve_commit(&self.repository, "HEAD")?.id();

		// Fetch + update the origin.
		self.repository
			.find_remote("origin")?
			.fetch::<&str>(&[], None, None)?;

		// Check if we've got a new HEAD.
		let new_head = resolve_commit(&self.repository, "HEAD")?.id();

		Ok(new_head != old_head)
	}

	/// Fetch a `Specifier` that matches the specified git reference and game version as closely as possible.
	pub fn specifier(&self, reference: &str, game_version: &str) -> Result<Specifier> {
		(reference, game_version).try_into_specifier(self)
	}

	/// Fetch the specified version of the schema.
	pub fn version(&self, specifier: impl TryIntoSpecifier) -> Result<Version> {
		let specifier = specifier.try_into_specifier(self)?;

		Ok(Version::new(Arc::clone(&self.repository), specifier))
	}
}

/// Configuration builder for EXDSchema provider.
#[derive(Debug)]
pub struct ProviderOptions {
	remote: Option<String>,
	directory: Option<PathBuf>,
}

impl ProviderOptions {
	fn new() -> Self {
		Self {
			remote: None,
			directory: None,
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
