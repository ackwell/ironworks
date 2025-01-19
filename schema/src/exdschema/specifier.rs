use std::{
	path::{Path, PathBuf},
	sync::LazyLock,
};

use git2::{ErrorCode, Oid};
use regex::Regex;

use crate::{
	error::{Error, ErrorValue, Result},
	git::resolve_commit,
};

use super::provider::Provider;

/// Canonical specifier for a schema version.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Specifier {
	V1(SpecifierV1),
	V2(SpecifierV2),
}

impl Specifier {
	pub(super) fn v1(provider: &Provider, revision: &str, game_version: &str) -> Result<Self> {
		let v1 = SpecifierV1::new(provider, revision, game_version)?;
		Ok(Self::V1(v1))
	}

	pub(super) fn v2_rev(provider: &Provider, revision: &str) -> Result<Self> {
		let v2 = SpecifierV2::from_rev(provider, revision)?;
		Ok(Self::V2(v2))
	}

	pub(super) fn v2_ver(provider: &Provider, game_version: &str) -> Result<Self> {
		let v2 = SpecifierV2::from_ver(provider, game_version)?;
		Ok(Self::V2(v2))
	}

	pub(super) fn commit(&self) -> Oid {
		match self {
			Self::V1(v1) => v1.commit,
			Self::V2(v2) => v2.commit,
		}
	}

	pub(super) fn sheet_path(&self, sheet: &str) -> PathBuf {
		match self {
			Self::V1(v1) => v1.sheet_path(sheet),
			Self::V2(v2) => v2.sheet_path(sheet),
		}
	}
}

/// Canonical specifier for a version 1 exdschema version.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecifierV1 {
	// TODO: would be neat to make this either an oid or full commit, but doing so would infect this struct with repository lifetimes
	commit: Oid,
	game_version: String,
}

impl SpecifierV1 {
	fn new(provider: &Provider, revision: &str, game_version: &str) -> Result<Self> {
		let repository = provider.repository.lock().unwrap();

		// Resolve the ref into a commit.
		let commit = resolve_commit(&repository, revision)?;

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

		Ok(Self {
			commit: commit.id(),
			game_version: found_version.into(),
		})
	}

	fn sheet_path(&self, sheet: &str) -> PathBuf {
		["Schemas", &self.game_version, &format!("{sheet}.yml")]
			.iter()
			.collect()
	}

	/// Get a string representative of this specifier's repository revision.
	pub fn revision(&self) -> String {
		self.commit.to_string()
	}

	/// Get a string representing the game version targeted by this specifier.
	pub fn game_version(&self) -> String {
		self.game_version.clone()
	}
}

/// Canonical specifier for a version 2 exdschema version.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecifierV2 {
	commit: Oid,
}

impl SpecifierV2 {
	fn from_rev(provider: &Provider, revision: &str) -> Result<Self> {
		let repository = provider.repository.lock().unwrap();
		let commit = resolve_commit(&repository, revision)?;

		// Validate that this is actually a v2 commit.
		match commit.tree()?.get_path(Path::new("schemas")) {
			Err(error) if error.code() == ErrorCode::NotFound => {
				return Err(Error::NotFound(ErrorValue::Version(revision.to_string())))
			}
			other => other?,
		};

		Ok(Self {
			commit: commit.id(),
		})
	}

	fn from_ver(provider: &Provider, game_version: &str) -> Result<Self> {
		// NOTE: This intentionally does not use ^/$, in case a prefix/suffix is
		// added in future.
		// NOTE: This uses [0-9] rather than \d as a pointlessly mico-optimisation
		// to avoid unicode character classes.
		static VERSION_BRANCH: LazyLock<Regex> = LazyLock::new(|| {
			// NNNN.NN.NN.NNNN.NNNN
			Regex::new(r"[0-9]{4}\.[0-9]{2}\.[0-9]{2}\.[0-9]{4}\.[0-9]{4}")
				.expect("regex construction should not fail")
		});

		let repository = provider.repository.lock().unwrap();

		let mut branches = repository
			.branches(Some(git2::BranchType::Local))?
			.filter_map(|res| {
				res.and_then(|(branch, _type)| {
					let name = branch.name()?.unwrap_or("");
					match VERSION_BRANCH.is_match(name) {
						true => Ok(Some((name.to_string(), branch))),
						false => Ok(None),
					}
				})
				.transpose()
			})
			.collect::<Result<Vec<_>, _>>()?;

		// We want the game versions latest-first.
		// ASSUMPTION: Game version strings are string-sortable.
		branches.sort_unstable_by(|(a, _), (b, _)| a.cmp(b).reverse());

		let (_name, branch) = branches
			.into_iter()
			.find(|(name, _branch)| name.as_str() <= game_version)
			.ok_or_else(|| Error::NotFound(ErrorValue::Version(game_version.into())))?;
		let oid = branch.get().peel_to_commit()?.id();

		Ok(Self { commit: oid })
	}

	fn sheet_path(&self, sheet: &str) -> PathBuf {
		["schemas", &format!("{sheet}.yml")].iter().collect()
	}

	/// Get a string representative of this specifier's repository revision.
	pub fn revision(&self) -> String {
		self.commit.to_string()
	}
}
