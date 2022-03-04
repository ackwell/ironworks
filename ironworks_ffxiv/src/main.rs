// This file exists as a temporary runner only
use std::{error::Error, fs, path::PathBuf};

use ironworks_sqpack::{Category, Repository, SqPack};

const TRY_PATHS: [&str; 5] = [
	"C:\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn",
	"C:\\Program Files (x86)\\Steam\\steamapps\\common\\FINAL FANTASY XIV Online",
	"C:\\Program Files (x86)\\Steam\\steamapps\\common\\FINAL FANTASY XIV - A Realm Reborn",
	"C:\\Program Files (x86)\\FINAL FANTASY XIV - A Realm Reborn",
	"C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn",
];

const WSL_PREFIX: [&str; 2] = ["/mnt", "c"];

const SQPACK_PATH: [&str; 2] = ["game", "sqpack"];

fn main() -> Result<(), Box<dyn Error>> {
	// TODO: allow override
	let mut install = find_install().unwrap();
	for segment in SQPACK_PATH {
		install.push(segment);
	}

	let sqpack = SqPack::new(
		String::from("ffxiv"),
		[Repository {
			id: 0,
			name: String::from("ffxiv"),
			path: install.join("ffxiv"),
		}],
		[Category {
			id: 0x0A,
			name: String::from("exd"),
		}],
	);

	let file_buffer = sqpack.read_file("exd/root.exl")?;

	let exlt = String::from_utf8(file_buffer).unwrap();

	println!("EXLT: {}", exlt);

	Ok(())
}

fn find_install() -> Option<PathBuf> {
	return TRY_PATHS
		.iter()
		.flat_map(|path| {
			[
				PathBuf::from(path),
				WSL_PREFIX
					.into_iter()
					.chain(path.split('\\').skip(1))
					.collect::<PathBuf>(),
			]
		})
		.find(|path| fs::metadata(path).is_ok())
		.map(PathBuf::from);
}
