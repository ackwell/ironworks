use std::{
	borrow::Cow,
	env::current_exe,
	path::{Path, PathBuf},
};

use derivative::Derivative;
use git2::{build::RepoBuilder, Repository};

use crate::error::{Error, ErrorValue, Result};

use super::version::Version;

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
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Provider {
	#[derivative(Debug = "ignore")]
	repository: Repository,
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
			.ok_or_else(|| Error::NotFound(ErrorValue::Other("Repository directory".into())))?;

		let repository = match directory.exists() {
			true => open_repository(remote, &directory),
			false => clone_repository(remote, &directory),
		}?;

		Ok(Self { repository })
	}

	/// Fetch the specified version of the schema.
	pub fn version(&self, version: &str) -> Result<Version> {
		let commit = self.repository.revparse_single(version)?.peel_to_commit()?;
		Ok(Version::new(commit))
	}
}

// TODO: Maybe... lazy static this? Would that work?
fn default_directory<'a>() -> Option<Cow<'a, Path>> {
	let path = current_exe().ok()?.parent()?.join(REPOSITORY_DIRECTORY);
	Some(path.into())
}

fn clone_repository(remote: &str, directory: &Path) -> Result<Repository> {
	let repository = RepoBuilder::new()
		.bare(true)
		.remote_create(|repo, name, url| repo.remote_with_fetch(name, url, "+refs/*:refs/*"))
		.clone(remote, directory)?;
	Ok(repository)
}

fn open_repository(remote: &str, directory: &Path) -> Result<Repository> {
	let repository = Repository::open_bare(&directory)?;
	if repository.find_remote("origin")?.url() != Some(remote) {
		return Err(Error::Repository(format!(
			"Repository at {directory:?} exists, does not have origin {remote}."
		)));
	}
	Ok(repository)
}
