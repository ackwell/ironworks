// This file exists as a temporary runner only
use std::{
	ffi,
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

fn main() -> anyhow::Result<()> {
	let sqpack = SqPack::ffxiv()?;

	let file_buffer = sqpack.read_file("exd/root.exl")?;
	let exlt = String::from_utf8(file_buffer)?;

	println!("EXLT: {}", exlt);

	Ok(())
}

trait SqPackFfxiv {
	fn ffxiv() -> Result<Self, Error>
	where
		Self: Sized;

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
		let install_path: PathBuf = path
			.iter()
			.chain(SQPACK_PATH.iter().map(|s| ffi::OsStr::new(*s)))
			.collect();

		Self::new(
			"ffxiv".into(),
			[Repository {
				id: 0,
				name: "ffxiv".into(),
				path: install_path.join("ffxiv"),
			}],
			[Category {
				id: 0x0A,
				name: "exd".into(),
			}],
		)
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
		.find(|p| p.exists())
		.map(PathBuf::from)
}
