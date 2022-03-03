use std::path::PathBuf;

use crate::sqpack::{Category, Repository};

pub fn build_file_path(
	repository: &Repository,
	category: &Category,
	chunk_id: u8,
	platform: &str,
	file_type: &str,
) -> PathBuf {
	let mut path = PathBuf::new();
	path.push(&repository.path);
	path.push(format!(
		"{:02x}{:02x}{:02x}.{}.{}",
		category.id, repository.id, chunk_id, platform, file_type
	));
	return path;
}
