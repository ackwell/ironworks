use std::path::PathBuf;

#[derive(Debug)]
pub struct PatchRepository {
	pub base_directory: PathBuf,
	pub patches: Vec<String>,
}

impl PatchRepository {
	// TODO: from_dir, plus From<PathBuf> that calls it
}
