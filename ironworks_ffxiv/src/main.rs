// This file exists as a temporary runner only
use std::{
	ffi,
	path::{self, Path, PathBuf},
};

use anyhow::Context;
use ironworks_sqpack::{Category, Repository, SqPack};

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
	let sqpack = SqPack::ffxiv();

	let file_buffer = sqpack.read_file("exd/root.exl")?;
	let exlt = String::from_utf8(file_buffer)?;

	println!("EXLT: {}", exlt);

	Ok(())
}

trait SqPackFfxiv {
	fn ffxiv() -> Self;
	fn ffxiv_at(path: &Path) -> Self;
}

impl SqPackFfxiv for SqPack<'_> {
	fn ffxiv() -> Self {
		// TODO: error handling
		// TODO: should i just inline find_install here
		Self::ffxiv_at(&find_install().unwrap())
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
