// This file exists as a temporary runner only
use std::{error::Error, fs, path::PathBuf};

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

fn main() -> Result<(), Box<dyn Error>> {
	// TODO: allow override
	let mut install = find_install().unwrap();
	for segment in SQPACK_PATH {
		install.push(segment);
	}

	let sqpack = SqPack::new(
		"ffxiv".into(),
		[Repository {
			id: 0,
			name: "ffxiv".into(),
			path: install.join("ffxiv"),
		}],
		[Category {
			id: 0x0A,
			name: "exd".into(),
		}],
	);

	let file_buffer = sqpack.read_file("exd/root.exl")?;

	let exlt = String::from_utf8(file_buffer).unwrap();

	println!("EXLT: {}", exlt);

	Ok(())
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
