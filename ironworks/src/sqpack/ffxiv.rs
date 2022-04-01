use std::{
	ffi::OsStr,
	fs, io,
	path::{Path, PathBuf},
};

use crate::error::{Error, ErrorValue, Result};

use super::resource::Resource;

// TODO: should this and the core resource get moved to a "resource" module by themselves?
//       do we expect this to also implement the sqpack extension?

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

	fn build_file_path(
		&self,
		repository: u8,
		category: u8,
		chunk: u8,
		extension: &str,
	) -> Result<PathBuf> {
		// TODO: Platform?
		let repository_name = self
			.repositories
			.get(usize::from(repository))
			.ok_or_else(|| {
				Error::NotFound(ErrorValue::Other(format!("repository {repository}")))
			})?;

		let file_name = format!("{category:02x}{repository:02x}{chunk:02x}.win32.{extension}");

		let file_path = self
			.path
			.join([repository_name, &file_name].iter().collect::<PathBuf>());

		Ok(file_path)
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

	type Index = io::Cursor<Vec<u8>>;
	fn index(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index> {
		read_index(self.build_file_path(repository, category, chunk, "index")?)
	}

	type Index2 = io::Cursor<Vec<u8>>;
	fn index2(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index2> {
		read_index(self.build_file_path(repository, category, chunk, "index2")?)
	}

	type Dat = fs::File;
	fn dat(&self, repository: u8, category: u8, chunk: u8, dat: u8) -> Result<Self::Dat> {
		let path = self.build_file_path(repository, category, chunk, &format!("dat{dat}"))?;
		fs::File::open(path).map_err(|error| Error::Resource(error.into()))
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

fn read_index(path: PathBuf) -> Result<io::Cursor<Vec<u8>>> {
	// Read the entire index into memory before returning - we typically need
	// the full dataset anyway, and working directly on a File causes significant
	// slowdowns due to IO syscalls.
	let buffer = fs::read(&path).map_err(|error| match error.kind() {
		io::ErrorKind::NotFound => Error::NotFound(ErrorValue::FilePath(path)),
		_ => Error::Resource(error.into()),
	})?;
	Ok(io::Cursor::new(buffer))
}
