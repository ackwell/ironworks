use std::{
	env::current_exe,
	path::{Path, PathBuf},
};

use git2::{build::RepoBuilder, Oid, Repository};

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

			log::trace!("Opened SaintCoinach at {:?}", directory);
			repository
		} else {
			log::info!("Cloning SaintCoinach from {} to {:?}", remote, directory);
			RepoBuilder::new().bare(true).clone(&remote, &directory)?
		};

		Ok(Self { repository })
	}

	fn version(&self, spec: &str) -> Result<SaintCoinachVersion> {
		let commit = self.repository.revparse_single(spec)?.peel_to_commit()?;
		Ok(SaintCoinachVersion {
			repository: &self.repository,
			commit,
		})
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

// this should impl a "version" trait or something
struct SaintCoinachVersion<'repo> {
	repository: &'repo Repository,
	commit: Commit<'repo>,
}

impl SaintCoinachVersion<'_> {
	// thoughts; for hash map keying & stuff
	fn id(&self) -> impl Eq + Hash + Display {
		self.commit.id()
	}

	// fn schemas -> iter

	fn schema(&self, sheet: &str) -> Result<()> {
		let definition_tree = self.temp_get_def_tree()?;
		// TODO: can probably skip this double-tap by having the tree lookup take Option<str> or something and return at the Object point, leave the object cast for the consumer
		let foo = definition_tree
			.get_name(&format!("{}.json", sheet))
			.expect("TODO HANDLE ME");
		let bar = foo.to_object(self.repository).expect("what is going on");
		let baz = bar.as_blob().expect("should be a blob");
		let qux = baz.content();
		println!("{}", String::from_utf8_lossy(qux));
		Ok(())
	}

	fn temp_get_def_tree(&self) -> Result<Tree<'_>, git2::Error> {
		let tree = self
			.commit
			.tree()?
			.get_path(Path::new("SaintCoinach/Definitions"))?
			.to_object(self.repository)?
			.into_tree()
			.expect("SHIT IS VERY BROKEN");
		Ok(tree)
	}
}

pub fn test() {
	let schema = SaintCoinachSchema::new().unwrap();
	// let version = schema.version("69caa7e14fed1caaeb2089fad484c25e491d3c37").unwrap();
	// let version = schema.version("69caa7e14fed1caaeb2089").unwrap();
	// let version = schema.version("refs/tags/69caa7e").unwrap();
	let version = schema.version("HEAD").unwrap();
	// let version = schema.version("master").unwrap();

	version.schema("AOZBoss").unwrap();
}
