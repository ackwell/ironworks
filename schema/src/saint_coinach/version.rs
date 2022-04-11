use git2::Commit;

/// A single version of the SaintCoinach schema.
#[derive(Debug)]
pub struct Version<'repo> {
	commit: Commit<'repo>,
}

impl<'repo> Version<'repo> {
	pub(super) fn new(commit: Commit<'repo>) -> Self {
		Version { commit }
	}
}
