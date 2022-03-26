use std::{env::current_exe, path::PathBuf};

use git2::{build::RepoBuilder, Repository};

use crate::{
	error::{Error, Result},
	version::SaintCoinachVersion,
};

// Default configuration
const REPOSITORY_URL: &str = "https://github.com/xivapi/SaintCoinach.git";
const REPOSITORY_DIRECTORY: &str = "saint_coinach";

// need to build some trait that represents what a "schema provider" looks like (ref manacutter probably)
// impl of that trait for stc can probably own a repository ref and do lazy lookups into the object db
// given how git works a canonical version is easy to trip a "need to update" check but will need to consider how to trip an update check for a ref like "HEAD"

// todo: name?
#[derive(Debug)]
pub struct SaintCoinachSchemaOptions {
	remote: Option<String>,
	directory: Option<PathBuf>,
}

impl SaintCoinachSchemaOptions {
	pub fn new() -> Self {
		SaintCoinachSchemaOptions {
			remote: None,
			directory: None,
		}
	}

	pub fn remote(&mut self, remote: impl ToString) -> &mut Self {
		self.remote = Some(remote.to_string());
		self
	}

	pub fn directory(&mut self, directory: impl Into<PathBuf>) -> &mut Self {
		self.directory = Some(directory.into());
		self
	}

	#[inline]
	pub fn build(&self) -> Result<SaintCoinachSchema> {
		SaintCoinachSchema::with_options(self)
	}
}

impl Default for SaintCoinachSchemaOptions {
	fn default() -> Self {
		Self::new()
	}
}

// TODO: can't derive debug on this due to repo - look into crates like `derivative` to handle?
pub struct SaintCoinachSchema {
	repository: Repository,
}

impl SaintCoinachSchema {
	#[inline]
	pub fn new() -> Result<Self> {
		Self::with_options(&Self::options())
	}

	#[inline]
	pub fn options() -> SaintCoinachSchemaOptions {
		SaintCoinachSchemaOptions::new()
	}

	pub fn with_options(options: &SaintCoinachSchemaOptions) -> Result<Self> {
		// todo: look into fs::canonicalize but it sounds like it only works for pre-existing stuff
		let directory = options
			.directory
			.clone()
			.or_else(default_directory)
			.ok_or_else(|| {
				Error::NotFound(
					"No directory was provided, and default directory could not be resolved."
						.to_string(),
				)
			})?;

		let remote = options
			.remote
			.clone()
			.unwrap_or_else(|| REPOSITORY_URL.to_string());

		let repository = if directory.exists() {
			let repository = Repository::open_bare(&directory)?;
			// If the pre-existing repository points to an origin we didn't expect,
			// fail out now so it doesn't do something weird later.
			match repository.find_remote("origin")?.url() {
				Some(url) if url == remote => (),
				url => {
					return Err(Error::Repository(format!(
						"Repository at {:?} has origin {}, expected {}.",
						&directory,
						url.unwrap_or("(none)"),
						remote
					)))
				}
			}

			log::trace!("Opened SaintCoinach at {:?}", directory);
			repository
		} else {
			log::info!("Cloning SaintCoinach from {} to {:?}", remote, directory);
			RepoBuilder::new().bare(true).clone(&remote, &directory)?
		};

		Ok(Self { repository })
	}

	pub fn version(&self, spec: &str) -> Result<SaintCoinachVersion> {
		let commit = self.repository.revparse_single(spec)?.peel_to_commit()?;
		Ok(SaintCoinachVersion::new(&self.repository, commit))
	}
}

fn default_directory() -> Option<PathBuf> {
	match current_exe() {
		Ok(path) => path
			.parent()
			.map(|parent| parent.join(REPOSITORY_DIRECTORY)),
		Err(_) => None,
	}
}
