use std::path::Path;

use git2::Oid;

use crate::{
	error::{Error, ErrorValue, Result},
	git::resolve_commit,
};

use super::provider::Provider;

/// Canonical specifier for a schema version.
/// Represents a game version at a schema commit.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Specifier {
	// TODO: would be neat to make this either an oid or full commit, but doing so would infect this struct with repository lifetimes
	pub(super) commit: Oid,
	pub(super) game_version: String,
}

impl Specifier {
	pub(super) fn new(provider: &Provider, reference: &str, game_version: &str) -> Result<Self> {
		let repository = provider.repository.lock().unwrap();

		// Resolve the ref into a commit.
		let commit = resolve_commit(&repository, reference)?;

		// Fetch the schema list for the given commit.
		let mut schemas = commit
			.tree()?
			.get_path(Path::new("Schemas"))?
			.to_object(&repository)?
			.peel_to_tree()?
			.into_iter()
			.filter_map(|entry| entry.name().map(|name| name.to_string()))
			.collect::<Vec<_>>();

		// We want the game versions latest-first.
		// ASSUMPTION: Game version strings are string-sortable.
		schemas.sort_unstable();
		schemas.reverse();

		// Find the latest known game version that is at most the requested game version.
		let found_version = schemas
			.into_iter()
			.find(|version| version.as_str() <= game_version)
			.ok_or_else(|| Error::NotFound(ErrorValue::Version(game_version.into())))?;

		Ok(Specifier {
			commit: commit.id(),
			game_version: found_version.into(),
		})
	}

	/// Get a string representative of this specifier's repository reference.
	pub fn reference(&self) -> String {
		self.commit.to_string()
	}

	/// Get a string representing the game version targeted by this specifier.
	pub fn game_version(&self) -> String {
		self.game_version.clone()
	}
}
