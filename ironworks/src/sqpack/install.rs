use std::{
	ffi::OsStr,
	fs,
	io::{self, Seek},
	path::{Path, PathBuf},
};

use crate::utility::{TakeSeekable, TakeSeekableExt};

use super::{
	Location, Resource,
	error::{Error, Result},
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

#[allow(dead_code)]
#[derive(Debug)]
enum Platform {
	Win32 = 0,
	PS3 = 1,
	PS4 = 2,
}

/// SqPack resource for reading game data from an on-disk FFXIV installation.
#[derive(Debug)]
pub struct Install {
	path: PathBuf,
	repositories: Vec<Option<String>>,
	platform: Platform,
}

impl Install {
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
			platform: Platform::Win32,
		}
	}

	fn build_file_path(
		&self,
		repository: u8,
		category: u8,
		chunk: u8,
		extension: &str,
	) -> Result<PathBuf> {
		let platform = match self.platform {
			Platform::Win32 => "win32",
			Platform::PS3 => todo!("PS3 platform"),
			Platform::PS4 => todo!("PS4 platform"),
		};

		let file_name = format!("{category:02x}{repository:02x}{chunk:02x}.{platform}.{extension}");

		let file_path = self.path.join(
			[self.get_repository_name(repository)?, &file_name]
				.iter()
				.collect::<PathBuf>(),
		);

		Ok(file_path)
	}

	fn get_repository_name(&self, repository: u8) -> Result<&String> {
		self.repositories
			.get(usize::from(repository))
			.and_then(|option| option.as_ref())
			.ok_or(Error::FileNotFound)
	}
}

impl Resource for Install {
	fn version(&self, repository: u8) -> Result<String> {
		let path = match repository {
			0 => self.path.join("..").join("ffxivgame.ver"),
			repo => {
				let repository_name = self.get_repository_name(repo)?;
				self.path
					.join(repository_name)
					.join(format!("{repository_name}.ver"))
			}
		};

		Ok(fs::read_to_string(path)?)
	}

	type Index = io::Cursor<Vec<u8>>;
	fn index(&self, repository: u8, category: u8, chunk: u8) -> Result<Option<Self::Index>> {
		read_index(self.build_file_path(repository, category, chunk, "index")?)
	}

	type Index2 = io::Cursor<Vec<u8>>;
	fn index2(&self, repository: u8, category: u8, chunk: u8) -> Result<Option<Self::Index2>> {
		read_index(self.build_file_path(repository, category, chunk, "index2")?)
	}

	type File = TakeSeekable<io::BufReader<fs::File>>;
	fn file(&self, repository: u8, category: u8, location: Location) -> Result<Self::File> {
		let path = self.build_file_path(
			repository,
			category,
			location.chunk(),
			&format!("dat{}", location.data_file()),
		)?;
		let mut file = io::BufReader::new(fs::File::open(path)?);

		let offset = u64::from(location.offset());
		// Resolve the size early in case we need to seek to find the end. Using
		// longhand here so I can shortcut seek failures.
		let size = match location.size() {
			Some(size) => u64::from(size),
			None => file.seek(io::SeekFrom::End(0))? - offset,
		};

		file.seek(io::SeekFrom::Start(offset))?;

		Ok(file.take_seekable(size)?)
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

fn find_repositories(path: &Path) -> Vec<Option<String>> {
	(0..=9)
		.map(|index| {
			let name = match index {
				0 => "ffxiv".into(),
				other => format!("ex{other}"),
			};

			path.join(&name).exists().then_some(name)
		})
		.collect()
}

fn read_index(path: PathBuf) -> Result<Option<io::Cursor<Vec<u8>>>> {
	// Read the entire index into memory before returning - we typically need
	// the full dataset anyway, and working directly on a File causes significant
	// slowdowns due to IO syscalls.
	match fs::read(&path) {
		Ok(buffer) => Ok(Some(io::Cursor::new(buffer))),
		Err(error) => match error.kind() {
			io::ErrorKind::NotFound => Ok(None),
			_ => Err(Error::Io(error)),
		},
	}
}
