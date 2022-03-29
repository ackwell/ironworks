use std::{
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

const CATEGORIES: &[Option<&str>] = &[
	/* 0x00 */ Some("common"),
	/* 0x01 */ Some("bgcommon"),
	/* 0x02 */ Some("bg"),
	/* 0x03 */ Some("cut"),
	/* 0x04 */ Some("chara"),
	/* 0x05 */ Some("shader"),
	/* 0x06 */ Some("ui"),
	/* 0x07 */ Some("sound"),
	/* 0x08 */ Some("vfx"),
	/* 0x09 */ Some("ui_script"),
	/* 0x0a */ Some("exd"),
	/* 0x0b */ Some("game_script"),
	/* 0x0c */ Some("music"),
	/* 0x0d */ None,
	/* 0x0e */ None,
	/* 0x0f */ None,
	/* 0x10 */ None,
	/* 0x11 */ None,
	/* 0x12 */ Some("sqpack_test"),
	/* 0x13 */ Some("debug"),
];

// TODO: should there be a ffxiv feature?
/// Resource adapter pre-configured to work with on-disk sqpack packages laid
/// out in the FFXIV format.
#[derive(Debug)]
pub struct FfxivFsResource {
	path: PathBuf,
	repositories: Vec<String>,
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
	fn path_metadata(&self, path: &str) -> Option<(u8, u8)> {
		let split = path.split('/').take(2).collect::<Vec<_>>();

		match split[..] {
			[path_category, path_repository] => Some((
				self.repositories
					.iter()
					.position(|repository| repository == path_repository)
					.unwrap_or(0)
					.try_into()
					.unwrap(),
				CATEGORIES
					.iter()
					.position(|category| category == &Some(path_category))?
					.try_into()
					.unwrap(),
			)),
			_ => None,
		}
	}

	type Index = std::io::Empty;
	fn index(&self, _repository: u8, _category: u8, _chunk: u8) -> Self::Index {
		std::io::empty()
	}

	type Index2 = std::io::Empty;
	fn index2(&self, _repository: u8, _category: u8, _chunk: u8) -> Self::Index {
		std::io::empty()
	}

	type Dat = std::io::Empty;
	fn dat(&self, _repository: u8, _category: u8, _chunk: u8) -> Self::Index {
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

fn find_repositories(path: &Path) -> Vec<String> {
	(0..=9)
		.filter_map(|index| {
			let name = if index == 0 {
				"ffxiv".into()
			} else {
				format!("ex{}", index)
			};

			if path.join(&name).exists() {
				Some(name)
			} else {
				None
			}
		})
		.collect()
}
