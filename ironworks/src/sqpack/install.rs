use std::{
	ffi::OsStr,
	fs::{self, File},
	io::{self},
	path::{Path, PathBuf},
};

use crate::{error::Result, utility::TakeSeekable};

use super::{
	Location, Resource,
	vinstall::{VInstall, Vfs},
};

const TRY_PATHS: &[&str] = &[
	r"C:\SquareEnix\FINAL FANTASY XIV - A Realm Reborn",
	r"C:\Program Files (x86)\Steam\steamapps\common\FINAL FANTASY XIV Online",
	r"C:\Program Files (x86)\Steam\steamapps\common\FINAL FANTASY XIV - A Realm Reborn",
	r"C:\Program Files (x86)\FINAL FANTASY XIV - A Realm Reborn",
	r"C:\Program Files (x86)\SquareEnix\FINAL FANTASY XIV - A Realm Reborn",
];

const WSL_PREFIX: &[&str] = &["/mnt", "c"];

const SQPACK_PATH: &[&str] = &["game", "sqpack"];

#[derive(Debug)]
struct Disk(PathBuf);

impl Disk {
	pub fn path(&self) -> &Path {
		&self.0
	}
}

impl Vfs for Disk {
	type File = File;

	fn exists(&self, path: impl AsRef<Path>) -> bool {
		self.0.join(path).exists()
	}

	fn read_to_string(&self, path: impl AsRef<Path>) -> io::Result<String> {
		fs::read_to_string(self.0.join(path))
	}

	fn read(&self, path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
		fs::read(self.0.join(path))
	}

	fn open(&self, path: impl AsRef<Path>) -> io::Result<Self::File> {
		File::open(self.0.join(path))
	}
}

/// SqPack resource for reading game data from an on-disk FFXIV installation.
#[derive(Debug)]
pub struct Install(VInstall<Disk>);

impl Install {
	/// Search for a FFXIV install in common locations, configuring a resource
	/// instance with the found install, if any.
	pub fn search() -> Option<Self> {
		Some(Self::at(&find_install()?))
	}

	/// Configure a resource instance with an installation of FFXIV at the specified path.
	pub fn at(path: &Path) -> Self {
		Self::at_sqpack(
			path.iter()
				.chain(SQPACK_PATH.iter().map(|s| OsStr::new(*s)))
				.collect::<PathBuf>(),
		)
	}

	pub fn at_sqpack(sqpack_path: impl Into<PathBuf>) -> Self {
		let sqpack_path = sqpack_path.into();
		Self(VInstall::at_sqpack(Disk(sqpack_path)))
	}

	pub fn path(&self) -> &Path {
		self.0.vfs().path()
	}
}

impl Resource for Install {
	fn version(&self, repository: u8) -> Result<String> {
		self.0.version(repository)
	}

	type Index = io::Cursor<Vec<u8>>;
	fn index(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index> {
		self.0.index(repository, category, chunk)
	}

	type Index2 = io::Cursor<Vec<u8>>;
	fn index2(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index2> {
		self.0.index2(repository, category, chunk)
	}

	type File = TakeSeekable<io::BufReader<fs::File>>;
	fn file(&self, repository: u8, category: u8, location: Location) -> Result<Self::File> {
		self.0.file(repository, category, location)
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
