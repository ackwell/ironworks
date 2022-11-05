use std::{
	cmp::Ordering,
	fs,
	path::{Path, PathBuf},
};

use crate::error::Result;

#[derive(Debug)]
pub struct PatchRepository {
	pub base_directory: PathBuf,
	pub patches: Vec<String>,
}

impl PatchRepository {
	pub fn from_directory(repository_path: &Path) -> Result<Self> {
		let mut patches = fs::read_dir(repository_path)?
			.filter_map(|entry| {
				let patch_path = match entry {
					Err(err) => return Some(Err(err)),
					Ok(entry) => entry.path(),
				};

				let extension = patch_path.extension().and_then(|osstr| osstr.to_str());
				match patch_path.is_file() && extension == Some("patch") {
					// TODO: should this error if the string conversion fails? atm it just ->None's
					true => patch_path
						.file_stem()
						.and_then(|osstr| osstr.to_str())
						.map(|str| Ok(str.to_string())),

					false => None,
				}
			})
			.collect::<Result<Vec<_>, _>>()?;

		patches.sort_unstable_by(|a, b| sort_patches(a, b));

		Ok(Self {
			base_directory: repository_path.to_owned(),
			patches,
		})
	}
}

fn sort_patches(a: &str, b: &str) -> Ordering {
	match a[1..].cmp(&b[1..]) {
		// The prefix "type" character is only ever [D]IFF or [H]IST - HIST always sorts first.
		Ordering::Equal => a[0..1].cmp(&b[0..1]).reverse(),

		// The primary "version" portion of the patch name is string-sortable (Y.M.D.P.Rp, where [P]art, [R]evision, [p]art-but-for-HISTs).
		order => order,
	}
}

// Dumb trait helper because TryFrom's blanket impl prevents me using it.
pub trait IntoPatchRepository {
	fn into_repository(self) -> Result<PatchRepository>;
}

impl IntoPatchRepository for PatchRepository {
	fn into_repository(self) -> Result<PatchRepository> {
		Ok(self)
	}
}

impl<P: AsRef<Path>> IntoPatchRepository for P {
	fn into_repository(self) -> Result<PatchRepository> {
		PatchRepository::from_directory(self.as_ref())
	}
}
