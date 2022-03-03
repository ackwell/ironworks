use std::{
	collections::HashMap,
	fs,
	io::{self, Cursor},
};

use binrw::BinRead;

use crate::{
	crc::crc32,
	error::{Result, SqPackError},
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

#[derive(Debug)]
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
		return match build_index_table(repository, category, chunk_id)? {
			None => return Ok(None),
			Some((kind, table)) => Ok(Some(Self { kind, table })),
		};
	}

	pub fn get_file_location(&self, sqpack_path: &str) -> Result<&FileLocation> {
		// Drop down to index kind specific lookup logic
		return match self.kind {
			IndexKind::Index1 => self.get_index1_location(sqpack_path),
			_ => todo!(),
		};
	}

	fn get_index1_location(&self, sqpack_path: &str) -> Result<&FileLocation> {
		// Build a hash via combination of crc of the two segments.
		let (directory, filename) = sqpack_path
			.rsplit_once('/')
			.ok_or_else(|| SqPackError::InvalidPath(sqpack_path.to_owned()))?;

		let directory_hash = crc32(directory.as_bytes());
		let filename_hash = crc32(filename.as_bytes());

		let hash_key = (directory_hash as u64) << 32 | filename_hash as u64;

		return self
			.table
			.get(&hash_key)
			.ok_or_else(|| SqPackError::NotFound(sqpack_path.to_owned()));
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
		Err(error) => return Err(SqPackError::from(error)),
	};

	// Build the index-kind specific table
	let table = match buffer {
		(IndexKind::Index1, buffer) => (IndexKind::Index1, build_index1_table(buffer, chunk_id)?),
		_ => todo!(),
	};

	return Ok(Some(table));
}

fn build_index1_table(buffer: Vec<u8>, chunk_id: u8) -> Result<IndexTable> {
	// TODO: fix the name abiguity here somehow
	let index = file_struct::Index::read(&mut Cursor::new(buffer))
		.map_err(|_| SqPackError::InvalidData(format!("Index data in \"{}\"", "TODO",)))?;

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

	return Ok(table);
}

fn get_index_buffer(
	repository: &Repository,
	category: &Category,
	chunk_id: u8,
) -> io::Result<(IndexKind, Vec<u8>)> {
	// Try to load `.index`, falling back to `.index2`.
	// Disambiguating the file types via kind.
	return fs::read(build_file_path(
		repository, category, chunk_id, "win32", "index",
	))
	.map(|buffer| (IndexKind::Index1, buffer))
	.or_else(|_| {
		fs::read(build_file_path(
			repository, category, chunk_id, "win32", "index2",
		))
		.map(|buffer| (IndexKind::Index2, buffer))
	});
}
