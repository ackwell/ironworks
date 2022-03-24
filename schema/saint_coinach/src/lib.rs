use std::{
	env::current_exe,
	path::{Path, PathBuf},
};

use git2::{build::RepoBuilder, Repository};
use log::{info, trace};

// need to build some trait that represents what a "schema provider" looks like (ref manacutter probably)
// impl of that trait for stc can probably own a repository ref and do lazy lookups into the object db
// given how git works a canonical version is easy to trip a "need to update" check but will need to consider how to trip an update check for a ref like "HEAD"

// Default configuration
const REPOSITORY_URL: &str = "https://github.com/xivapi/SaintCoinach.git";
const REPOSITORY_DIRECTORY: &str = "saint_coinach";

#[derive(thiserror::Error, Debug)]
enum Error {
	// TODO: I should probably make the not found errors more data-y, like _what_ wasn't found _where_, etc.
	#[error("Not found: {0}")]
	NotFound(String),

	// TODO: This exposes the fact that we _use_ git, but not the impl details of git2. is that enough? is that too much? I'm not sure.
	#[error("Git error: {0}")]
	Git(String),
}

// TODO: aaaaaa i don't knoooow
impl From<git2::Error> for Error {
	fn from(error: git2::Error) -> Self {
		Error::Git(error.to_string())
	}
}

type Result<T, E = Error> = std::result::Result<T, E>;

// todo: name?
#[derive(Debug)]
struct SaintCoinachSchemaOptions {
	remote: Option<String>,
	directory: Option<PathBuf>,
}

impl SaintCoinachSchemaOptions {
	fn new() -> Self {
		SaintCoinachSchemaOptions {
			remote: None,
			directory: None,
		}
	}

	fn remote(&mut self, remote: impl ToString) -> &mut Self {
		self.remote = Some(remote.to_string());
		self
	}

	fn directory(&mut self, directory: impl Into<PathBuf>) -> &mut Self {
		self.directory = Some(directory.into());
		self
	}

	#[inline]
	fn build(&self) -> Result<SaintCoinachSchema> {
		SaintCoinachSchema::with_options(self)
	}
}

// TODO: can't derive debug on this due to repo - look into crates like `derivative` to handle?
struct SaintCoinachSchema {
	repository: Repository,
}

impl SaintCoinachSchema {
	#[inline]
	fn new() -> Result<Self> {
		Self::with_options(&Self::options())
	}

	#[inline]
	fn options() -> SaintCoinachSchemaOptions {
		SaintCoinachSchemaOptions::new()
	}

	fn with_options(options: &SaintCoinachSchemaOptions) -> Result<Self> {
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
					return Err(Error::Git(format!(
						"Repository at {:?} has origin {}, expected {}.",
						&directory,
						url.unwrap_or("(none)"),
						remote
					)))
				}
			}

			trace!("Opened SaintCoinach at {:?}", directory);
			repository
		} else {
			info!("Cloning SaintCoinach from {} to {:?}", remote, directory);
			RepoBuilder::new().bare(true).clone(&remote, &directory)?
		};

		Ok(Self { repository })
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

pub fn test() {
	let schema = SaintCoinachSchema::new().unwrap();

	// cool so construction is... dealt with. need to work out the api. having some way to canonicalise a "version" into a true version is important for yes reasons
	// given we probably want to trait most of this, perhaps a trait + struct impl - for stc impl it can probably be a wrapper around a git2 Oid?
	let commit = schema
		.repository
		.find_reference("HEAD")
		.unwrap()
		.peel_to_commit()
		.unwrap();

	let definition_tree = commit
		.tree()
		.unwrap()
		.get_path(Path::new("SaintCoinach/Definitions"))
		.unwrap()
		.to_object(&schema.repository)
		.unwrap()
		.into_tree()
		.unwrap();

	definition_tree
		.iter()
		.for_each(|e| println!("{:?}", e.name()))
}
