use std::path::Path;

use git2::{build::RepoBuilder, Commit, Direction, ErrorCode, Repository};

use crate::error::{Error, ErrorValue, Result};

pub fn open_repository(remote: &str, directory: &Path) -> Result<Repository> {
	match Repository::open_bare(directory) {
		Ok(repository) => validate_repository(repository, remote, directory),
		Err(error) => match error.code() {
			ErrorCode::NotFound => clone_repository(remote, directory),
			_ => Err(error)?,
		},
	}
}

fn validate_repository(
	repository: Repository,
	remote: &str,
	directory: &Path,
) -> Result<Repository> {
	if repository.find_remote("origin")?.url() != Some(remote) {
		return Err(Error::Repository(format!(
			"repository at {directory:?} exists, does not have expected origin {remote}"
		)));
	}

	Ok(repository)
}

fn clone_repository(remote: &str, directory: &Path) -> Result<Repository> {
	let mut default_branch = None;

	let repository = RepoBuilder::new()
		.bare(true)
		.remote_create(|repository, name, url| {
			// Use a mirror refspec - we want to target arbitrary references.
			let mut remote = repository.remote_with_fetch(name, url, "+refs/*:refs/*")?;

			// Connect and resolve the default branch used by the remote.
			remote.connect(Direction::Fetch)?;
			let branch_name = remote.default_branch()?;
			default_branch = Some(branch_name);

			Ok(remote)
		})
		.clone(remote, directory)?;

	// If we were able to retrieve a default branch from the remote, set it as our
	// HEAD. This avoids potentially falling back to `init.defaultbranch`, which
	// may not align with a branch on the remote.
	if let Some(branch_name) = default_branch {
		repository.set_head_bytes(&branch_name)?;
	}

	Ok(repository)
}

pub fn resolve_commit<'repo>(
	repository: &'repo Repository,
	reference: impl AsRef<str>,
) -> Result<Commit<'repo>> {
	let reference = reference.as_ref();
	repository
		.revparse_single(reference)
		.and_then(|object| object.peel_to_commit())
		.map_err(|error| match error.code() {
			// NotFound stems from invalid input to revparse, and InvalidSpec is
			// from a valid object reference that did not point to a commit.
			ErrorCode::NotFound | ErrorCode::InvalidSpec => {
				Error::NotFound(ErrorValue::Version(reference.into()))
			}
			_ => Error::from(error),
		})
}
