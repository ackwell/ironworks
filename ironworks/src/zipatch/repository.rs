use std::{
	cmp::Ordering,
	fs,
	path::{Path, PathBuf},
};

use crate::error::Result;

/// Representation of a folder containing patch files.
#[derive(Debug)]
pub struct PatchRepository {
	/// The filesystem path to the folder containing the patch files.
	pub base_directory: PathBuf,
	/// List of patch names. This should _not_ include the `.patch` suffix.
	pub patches: Vec<String>,
}

impl PatchRepository {
	/// Read a patch repository from the specified path. Patches will be sorted
	/// following the FFXIV patch ordering.
	pub fn at(repository_path: &Path) -> Result<Self> {
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
