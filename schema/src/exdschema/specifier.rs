use std::path::Path;

use git2::Oid;

use crate::{
	error::{Error, ErrorValue, Result},
	git::resolve_commit,
};

use super::provider::Provider;

/// Canonical specifier for a schema version.
/// Represents a game version at a schema commit.
#[derive(Debug)]
pub struct Specifier {
	// TODO: would be neat to make this either an oid or full commit, but doing so would infect this struct with repository lifetimes
	pub(super) commit: Oid,
	pub(super) game_version: String,
}

/// Trait implemented for types that can be converted to a `Specifier`.
pub trait TryIntoSpecifier: Sized {
	/// Convert the type into a `Specifier`.
	fn try_into_specifier(self, provider: &Provider) -> Result<Specifier>;
}

impl TryIntoSpecifier for Specifier {
	fn try_into_specifier(self, _provider: &Provider) -> Result<Specifier> {
		Ok(self)
	}
}

impl<R, V> TryIntoSpecifier for (R, V)
where
	R: AsRef<str>,
	V: AsRef<str>,
{
	fn try_into_specifier(self, provider: &Provider) -> Result<Specifier> {
		let (reference, game_version) = (self.0.as_ref(), self.1.as_ref());

		// Resolve the ref into a commit.
		let commit = resolve_commit(&provider.repository, reference)?;

		// Fetch the schema list for the given commit.
		let mut schemas = commit
			.tree()?
			.get_path(Path::new("Schemas"))?
			.to_object(&provider.repository)?
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
}
