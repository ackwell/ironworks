use std::{
	borrow::Cow,
	env::current_exe,
	path::{Path, PathBuf},
};

use derivative::Derivative;
use git2::Repository;

use crate::{
	error::{Error, ErrorValue, Result},
	git::open_repository,
};

use super::{specifier::Specifier, IntoSpecifier};

const DEFAULT_REMOTE: &str = "https://github.com/xivdev/EXDSchema.git";
const DEFAULT_DIRECTORY: &str = "exdschema";

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Provider {
	#[derivative(Debug = "ignore")]
	pub(super) repository: Repository,
}

impl Provider {
	pub fn new() -> Result<Self> {
		Self::with().build()
	}

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

		Ok(Self { repository })
	}

	// this works
	pub fn update(&self) -> Result<()> {
		self.repository
			.find_remote("origin")?
			.fetch::<&str>(&[], None, None)?;

		// todo: need to flag in some way that an update occured?
		//       a success does not mean anything changed, just that the attempt didn't fail

		Ok(())
	}

	// TODO: should this accept intospecifier? would allow it to be used as a one-stop canonicalisation pass
	pub fn specifier(&self, reference: &str, game_version: &str) -> Result<Specifier> {
		(reference, game_version).into_specifier(self)
	}

	// get version (impl into specifier? - two axes make this a little hard unless it's over a tuple)
	// pub fn version(specifier: impl IntoSpecifier) {
	// 	let specifier = specifier.into_specifier();
	// }
}

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

	pub fn remote(mut self, remote: impl Into<String>) -> Self {
		self.remote = Some(remote.into());
		self
	}

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
