use std::{
	ffi::OsStr,
	path::{Path, PathBuf},
};

use ironworks_sqpack::{Category, Error, Repository, SqPack};

const TRY_PATHS: &[&str] = &[
	"C:\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn",
	"C:\\Program Files (x86)\\Steam\\steamapps\\common\\FINAL FANTASY XIV Online",
	"C:\\Program Files (x86)\\Steam\\steamapps\\common\\FINAL FANTASY XIV - A Realm Reborn",
	"C:\\Program Files (x86)\\FINAL FANTASY XIV - A Realm Reborn",
	"C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn",
];

const WSL_PREFIX: &[&str] = &["/mnt", "c"];

const SQPACK_PATH: &[&str] = &["game", "sqpack"];

const CATEGORIES: &[(&str, u8)] = &[
	("common", 0x00),
	("bgcommon", 0x01),
	("bg", 0x02),
	("cut", 0x03),
	("chara", 0x04),
	("shader", 0x05),
	("ui", 0x06),
	("sound", 0x07),
	("vfx", 0x08),
	("ui_script", 0x09),
	("exd", 0x0a),
	("game_script", 0x0b),
	("music", 0x0c),
	("sqpack_test", 0x12),
	("debug", 0x13),
];

/// Extension trait that adds methods to construct SqPack instances pre-configured
/// for FFXIV data.
pub trait SqPackFfxiv {
	/// Search for a FFXIV install in common locations (on Windows), and configure
	/// a SqPack instance with the found install, if any.
	fn ffxiv() -> Result<Self, Error>
	where
		Self: Sized;

	/// Configure a SqPack instance with an installation of FFXIV at the specified path.
	fn ffxiv_at(path: &Path) -> Self;
}

impl SqPackFfxiv for SqPack<'_> {
	fn ffxiv() -> Result<Self, Error> {
		let path = find_install().ok_or_else(|| {
			Error::InvalidDatabase(
				"Could not find install in common locations, please provide a path.".into(),
			)
		})?;
		Ok(Self::ffxiv_at(&path))
	}

	fn ffxiv_at(path: &Path) -> Self {
		let sqpack_path = path
			.iter()
			.chain(SQPACK_PATH.iter().map(|s| OsStr::new(*s)))
			.collect::<PathBuf>();

		let repositories = find_repositories(&sqpack_path);

		let categories = CATEGORIES.iter().map(|(name, id)| Category {
			name: (*name).into(),
			id: *id,
		});

		Self::new("ffxiv".into(), repositories, categories)
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

fn find_repositories(sqpack_path: &Path) -> impl IntoIterator<Item = Repository> + '_ {
	(0..=9)
		.map(|index| {
			let name = if index == 0 {
				"ffxiv".into()
			} else {
				format!("ex{}", index)
			};
			Repository {
				id: index,
				path: sqpack_path.join(&name),
				name,
			}
		})
		.filter(|repository| repository.path.exists())
}
