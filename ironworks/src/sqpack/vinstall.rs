use std::{
	io::{self, Read, Seek},
	path::{Path, PathBuf},
};

use crate::{
	error::{Error, ErrorValue, Result},
	utility::{TakeSeekable, TakeSeekableExt},
};

use super::{Location, Resource};

#[allow(dead_code)]
#[derive(Debug)]
enum Platform {
	Win32 = 0,
	PS3 = 1,
	PS4 = 2,
}

pub trait Vfs {
	type File: Read + Seek;

	fn exists(&self, path: impl AsRef<Path>) -> bool;

	fn open(&self, path: impl AsRef<Path>) -> io::Result<Self::File>;

	fn read(&self, path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
		let mut buf = Vec::new();
		self.open(path)?.read_to_end(&mut buf)?;
		Ok(buf)
	}

	fn read_to_string(&self, path: impl AsRef<Path>) -> io::Result<String> {
		let mut buf = String::new();
		self.open(path)?.read_to_string(&mut buf)?;
		Ok(buf)
	}
}

/// SqPack resource for reading game data from an abstracted disk FFXIV installation.
#[derive(Debug)]
pub struct VInstall<V: Vfs> {
	vfs: V,
	repositories: Vec<Option<String>>,
	platform: Platform,
}

impl<V: Vfs> VInstall<V> {
	pub fn at_sqpack(vfs: V) -> Self {
		let repositories = find_repositories(&vfs);

		Self {
			vfs,
			repositories,
			platform: Platform::Win32,
		}
	}

	pub fn vfs(&self) -> &V {
		&self.vfs
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

		let file_path = [self.get_repository_name(repository)?, &file_name]
			.iter()
			.collect::<PathBuf>();

		Ok(file_path)
	}

	fn get_repository_name(&self, repository: u8) -> Result<&String> {
		self.repositories
			.get(usize::from(repository))
			.and_then(|option| option.as_ref())
			.ok_or_else(|| Error::NotFound(ErrorValue::Other(format!("repository {repository}"))))
	}
}

impl<V: Vfs> Resource for VInstall<V> {
	fn version(&self, repository: u8) -> Result<String> {
		let path = match repository {
			0 => PathBuf::new().join("..").join("ffxivgame.ver"),
			repo => {
				let repository_name = self.get_repository_name(repo)?;
				PathBuf::new()
					.join(repository_name)
					.join(format!("{repository_name}.ver"))
			}
		};

		Ok(self.vfs().read_to_string(path)?)
	}

	type Index = io::Cursor<Vec<u8>>;
	fn index(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index> {
		read_index(
			self.vfs(),
			self.build_file_path(repository, category, chunk, "index")?,
		)
	}

	type Index2 = io::Cursor<Vec<u8>>;
	fn index2(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index2> {
		read_index(
			self.vfs(),
			self.build_file_path(repository, category, chunk, "index2")?,
		)
	}

	type File = TakeSeekable<io::BufReader<V::File>>;
	fn file(&self, repository: u8, category: u8, location: Location) -> Result<Self::File> {
		let path = self.build_file_path(
			repository,
			category,
			location.chunk(),
			&format!("dat{}", location.data_file()),
		)?;
		let mut file = io::BufReader::new(self.vfs().open(path)?);

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

fn find_repositories(vfs: &impl Vfs) -> Vec<Option<String>> {
	(0..=9)
		.map(|index| {
			let name = match index {
				0 => "ffxiv".into(),
				other => format!("ex{other}"),
			};

			vfs.exists(Path::new(&name)).then_some(name)
		})
		.collect()
}

fn read_index(vfs: &impl Vfs, path: PathBuf) -> Result<io::Cursor<Vec<u8>>> {
	// Read the entire index into memory before returning - we typically need
	// the full dataset anyway, and working directly on a File causes significant
	// slowdowns due to IO syscalls.
	let buffer = vfs.read(&path).map_err(|error| match error.kind() {
		io::ErrorKind::NotFound => {
			Error::NotFound(ErrorValue::Other(format!("file path {path:?}")))
		}
		_ => Error::Resource(error.into()),
	})?;
	Ok(io::Cursor::new(buffer))
}
