use std::{
	cmp::Ordering,
	fs, io,
	path::{Path, PathBuf},
};

/// Representation of a single patch file.
#[derive(Debug)]
pub struct Patch {
	/// Canonical name of the patch. Typically conforms to the format Y.M.D.P.Rp,
	/// where \[Y]ear, \[M]onth, \[D]ay, \[P]art, \[R]evision, \[p]art-but-for-HISTs
	pub name: String,
	/// Path to the patch file on disk.
	pub path: PathBuf,
}

/// Representation of a folder containing patch files.
#[derive(Debug)]
pub struct PatchRepository {
	/// List of patches in this repository
	pub patches: Vec<Patch>,
}

impl PatchRepository {
	/// Read a patch repository from the specified path. Patches will be sorted
	/// following the FFXIV patch ordering.
	pub fn at(repository_path: &Path) -> io::Result<Self> {
		let mut patches = fs::read_dir(repository_path)?
			.filter_map(|entry| {
				let patch_path = match entry {
					Err(err) => return Some(Err(err)),
					Ok(entry) => entry.path(),
				};

				let extension = patch_path.extension().and_then(|osstr| osstr.to_str());

				if !(patch_path.is_file() && extension == Some("patch")) {
					return None;
				}

				// TODO: should this error if the string conversion fails? atm it just ->None's
				let name = patch_path.file_stem()?.to_str()?.to_string();

				Some(Ok(Patch {
					name,
					path: patch_path,
				}))
			})
			.collect::<Result<Vec<_>, _>>()?;

		patches.sort_unstable_by(sort_patches);

		Ok(Self { patches })
	}

	// TODO: fn before - so i.e. a simple use case can `.at().before()` to get a repo of a folder containing patches before a particular point.
}

fn sort_patches(Patch { name: a, .. }: &Patch, Patch { name: b, .. }: &Patch) -> Ordering {
	match a[1..].cmp(&b[1..]) {
		// The prefix "type" character is only ever [D]IFF or [H]IST - HIST always sorts first.
		Ordering::Equal => a[0..1].cmp(&b[0..1]).reverse(),

		// The primary "version" portion of the patch name is string-sortable.
		order => order,
	}
}
