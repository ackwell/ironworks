use std::{
	env::current_exe,
	path::{Path, PathBuf},
};

use git2::{build::RepoBuilder, Repository};

// need to build some trait that represents what a "schema provider" looks like (ref manacutter probably)
// impl of that trait for stc can probably own a repository ref and do lazy lookups into the object db
// given how git works a canonical version is easy to trip a "need to update" check but will need to consider how to trip an update check for a ref like "HEAD"

// Default configuration
const REPOSITORY_URL: &str = "https://github.com/xivapi/SaintCoinach.git";
const REPOSITORY_DIRECTORY: &str = "saint_coinach";

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
	fn build(&self) -> SaintCoinachSchema {
		SaintCoinachSchema::with_options(self)
	}
}

// TODO: can't derive debug on this due to repo - look into crates like `derivative` to handle?
struct SaintCoinachSchema {
	repository: Repository,
}

impl SaintCoinachSchema {
	#[inline]
	fn new() -> Self {
		Self::with_options(&Self::options())
	}

	#[inline]
	fn options() -> SaintCoinachSchemaOptions {
		SaintCoinachSchemaOptions::new()
	}

	fn with_options(options: &SaintCoinachSchemaOptions) -> Self {
		// todo: look into fs::canonicalize but it sounds like it only works for pre-existing stuff
		let directory = options.directory.clone().or_else(default_directory);

		// TODO: handle error
		let directory =
			directory.expect("dir not found or not provided or something fucky went wrong pls fix");

		let remote = options
			.remote
			.clone()
			.unwrap_or_else(|| REPOSITORY_URL.to_string());

		let repository = if directory.exists() {
			Repository::open_bare(directory)
		} else {
			RepoBuilder::new().bare(true).clone(&remote, &directory)
		};

		// TODO: handle error
		let repository = repository.unwrap();

		Self { repository }
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
	let schema = SaintCoinachSchema::new();

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

	// println!("reference {:?}", foo.kind());

	definition_tree
		.iter()
		.for_each(|e| println!("{:?}", e.name()))
}
