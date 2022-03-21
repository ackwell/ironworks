use std::{
	collections::HashMap,
	fs,
	io::{self, Cursor},
	path::PathBuf,
};

use binrw::BinRead;

use crate::{
	crc::crc32,
	error::{Error, Result},
	file_struct,
	sqpack::{Category, Repository},
	utility::build_file_path,
};

#[derive(Debug)]
pub struct FileLocation {
	pub chunk_id: u8,
	pub data_file_id: u8,
	pub offset: u32,
}

#[derive(Clone, Copy, Debug)]
enum IndexKind {
	Index1,
	Index2,
}

type IndexTable = HashMap<u64, FileLocation>;

#[derive(Debug)]
pub struct Index {
	kind: IndexKind,
	table: IndexTable,
}

impl Index {
	pub fn new(repository: &Repository, category: &Category, chunk_id: u8) -> Result<Option<Self>> {
		// Preemptively build the index table. If there is no table for this
		// configuration of sqpack, fail gracefully.
		Ok(build_index_table(repository, category, chunk_id)?
			.map(|(kind, table)| Self { kind, table }))
	}

	pub fn get_file_location(&self, sqpack_path: &str) -> Result<&FileLocation> {
		// Drop down to index kind specific lookup logic
		match self.kind {
			IndexKind::Index1 => self.get_index1_location(sqpack_path),
			IndexKind::Index2 => todo!(),
		}
	}

	fn get_index1_location(&self, sqpack_path: &str) -> Result<&FileLocation> {
		// Build a hash via combination of crc of the two segments.
		let (directory, filename) = sqpack_path
			.rsplit_once('/')
			.ok_or_else(|| Error::InvalidPath(sqpack_path.to_owned()))?;

		let directory_hash = crc32(directory.as_bytes());
		let filename_hash = crc32(filename.as_bytes());

		let hash_key = (directory_hash as u64) << 32 | filename_hash as u64;

		return self
			.table
			.get(&hash_key)
			.ok_or_else(|| Error::NotFound(sqpack_path.to_owned()));
	}
}

fn build_index_table(
	repository: &Repository,
	category: &Category,
	chunk_id: u8,
) -> Result<Option<(IndexKind, IndexTable)>> {
	// Try to get the buffer for the chunk's index. Special casing NotFound as a
	// successful lack of value as the majority of possible chunks will not exist.
	let buffer = match get_index_buffer(repository, category, chunk_id) {
		Ok(buffer) => buffer,
		Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
		Err(error) => return Err(Error::from(error)),
	};

	let kind = buffer.kind;
	let table = match kind {
		IndexKind::Index1 => build_index1_table(buffer, chunk_id)?,
		IndexKind::Index2 => todo!(),
	};

	Ok(Some((kind, table)))
}

fn build_index1_table(buffer: IndexBuffer, chunk_id: u8) -> Result<IndexTable> {
	// TODO: fix the name abiguity here somehow
	let index = file_struct::Index::read(&mut Cursor::new(buffer.buffer)).map_err(|error| {
		Error::InvalidDatabase(format!(
			"Erroneous index data in \"{}\": {}",
			buffer.path.to_string_lossy(),
			error
		))
	})?;

	// Build the lookup table
	let table: HashMap<_, _> = index
		.indexes
		.into_iter()
		.map(|entry| {
			(
				entry.hash,
				FileLocation {
					chunk_id,
					data_file_id: entry.value.data_file_id,
					offset: entry.value.offset,
				},
			)
		})
		.collect();

	Ok(table)
}

struct IndexBuffer {
	kind: IndexKind,
	path: PathBuf,
	buffer: Vec<u8>,
}

fn get_index_buffer(
	repository: &Repository,
	category: &Category,
	chunk_id: u8,
) -> io::Result<IndexBuffer> {
	// Try to load `.index`, falling back to `.index2`.
	// Disambiguating the file types via kind.
	let read_index = |kind, platform, file_type| {
		let path = build_file_path(repository, category, chunk_id, platform, file_type);
		fs::read(&path).map(|buffer| IndexBuffer { kind, path, buffer })
	};

	read_index(IndexKind::Index1, "win32", "index")
		.or_else(|_| read_index(IndexKind::Index2, "win32", "index2"))
}
