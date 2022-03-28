use std::{
	collections::HashMap,
	ffi::OsStr,
	path::{Path, PathBuf},
};

use super::resource::Resource;

const TRY_PATHS: &[&str] = &[
	r"C:\SquareEnix\FINAL FANTASY XIV - A Realm Reborn",
	r"C:\Program Files (x86)\Steam\steamapps\common\FINAL FANTASY XIV Online",
	r"C:\Program Files (x86)\Steam\steamapps\common\FINAL FANTASY XIV - A Realm Reborn",
	r"C:\Program Files (x86)\FINAL FANTASY XIV - A Realm Reborn",
	r"C:\Program Files (x86)\SquareEnix\FINAL FANTASY XIV - A Realm Reborn",
];

const WSL_PREFIX: &[&str] = &["/mnt", "c"];

const SQPACK_PATH: &[&str] = &["game", "sqpack"];

const DEFAULT_REPOSITORY: &str = "ffxiv";

// TODO: should there be a ffxiv feature?
/// Resource adapter pre-configured to work with on-disk sqpack packages laid
/// out in the FFXIV format.
#[derive(Debug)]
pub struct FfxivFsResource {
	path: PathBuf,
	repositories: HashMap<String, u8>,
}

impl FfxivFsResource {
	// TODO: should this error instead of option? i'm tempted to say it should for the sake of consumers
	/// Search for a FFXIV install in common locations, configuring a resource
	/// instance with the found install, if any.
	pub fn search() -> Option<Self> {
		Some(Self::at(&find_install()?))
	}

	/// Configure a resource instance with an installation of FFXIV at the specified path.
	pub fn at(path: &Path) -> Self {
		let sqpack_path = path
			.iter()
			.chain(SQPACK_PATH.iter().map(|s| OsStr::new(*s)))
			.collect::<PathBuf>();

		let repositories = find_repositories(&sqpack_path);

		Self {
			path: sqpack_path,
			repositories,
		}
	}
}

impl Resource for FfxivFsResource {
	fn path_metadata<'a>(&self, path: &'a str) -> Option<(&'a str, &'a str)> {
		let split = path.split('/').take(2).collect::<Vec<_>>();
		match split[..] {
			[category, repository] => Some((
				match self.repositories.contains_key(repository) {
					true => repository,
					false => DEFAULT_REPOSITORY,
				},
				category,
			)),
			_ => None,
		}
	}

	type Index = std::io::Empty;
	fn index(&self, _repository: &str, _category: &str, _chunk: u8) -> Self::Index {
		std::io::empty()
	}

	type Index2 = std::io::Empty;
	fn index2(&self, _repository: &str, _category: &str, _chunk: u8) -> Self::Index {
		std::io::empty()
	}

	type Dat = std::io::Empty;
	fn dat(&self, _repository: &str, _category: &str, _chunk: u8) -> Self::Index {
		std::io::empty()
	}
}

fn find_install() -> Option<PathBuf> {
	TRY_PATHS
		.iter()
		.flat_map(|path| {
			[
				PathBuf::from(path),
				WSL_PREFIX
					.iter()
					.copied()
					.chain(path.split('\\').skip(1))
					.collect::<PathBuf>(),
			]
		})
		.find(|path| path.exists())
}

fn find_repositories(path: &Path) -> HashMap<String, u8> {
	(0..=9)
		.filter_map(|index| {
			let name = if index == 0 {
				DEFAULT_REPOSITORY.into()
			} else {
				format!("ex{}", index)
			};

			if path.join(&name).exists() {
				Some((name, index))
			} else {
				None
			}
		})
		.collect()
}
