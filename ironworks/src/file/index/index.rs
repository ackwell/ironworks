use core::fmt;
use std::io::SeekFrom;

use binrw::{binread, BinRead};

use crate::{
	error::{Error, ErrorValue, Result},
	file::File,
	FileStream,
};

use super::crc::crc32;

#[binread]
#[derive(Debug)]
#[br(little)]
pub struct Index {
	#[br(temp)]
	sqpack_header: SqPackHeader,

	#[br(temp, seek_before = SeekFrom::Start(sqpack_header.size.into()))]
	index_header: IndexHeader,

	#[br(
		seek_before = SeekFrom::Start(index_header.index_data.offset.into()),
		count = index_header.index_data.size / Entry::SIZE,
	)]
	indexes: Vec<Entry>,
}

impl Index {
	pub fn find(&self, path: &str) -> Result<FileMetadata> {
		// Calculate the Index1 hash of the path
		let hashed_segments = path
			.rsplitn(2, '/')
			.map(|segment| crc32(segment.as_bytes()))
			.collect::<Vec<_>>();

		let hash = match hashed_segments[..] {
			[file, directory] => (directory as u64) << 32 | file as u64,
			_ => {
				return Err(Error::Invalid(
					ErrorValue::Path(path.into()),
					"Paths must contain at least two segments.".into(),
				))
			}
		};

		// Look for a matching entry in the index table
		// TODO: hashmap this probably
		// TODO: i saw a neat impl that was a pass-through hasher for a map to save time on hashing small values. maybe?
		self.indexes
			.iter()
			.find(|entry| entry.hash == hash)
			.map(|entry| entry.file_metadata.clone())
			.ok_or_else(|| Error::NotFound(ErrorValue::Path(path.into())))
	}
}

impl File for Index {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut stream)?)
	}
}

// TODO: A lot of this will be shared with index2 and/or datN, how do I execute that sharing?

#[binread]
#[derive(Debug)]
#[br(little, magic = b"SqPack\0\0")]
struct SqPackHeader {
	_platform_id: u8,
	// unknown: [u8; 3],
	#[br(pad_before = 3)]
	pub size: u32,
	_version: u32,
	_kind: u32,
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct IndexHeader {
	_size: u32,
	_version: u32,
	pub index_data: Section,
	_data_file_count: u32,
	_synonym_data: Section,
	_empty_block_data: Section,
	_dir_index_data: Section,
	_index_type: u32,

	#[br(pad_before = 656)] // reserved
	_digest: Digest,
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct Section {
	pub offset: u32,
	pub size: u32,
	_digest: Digest,
}

#[binread]
struct Digest([u8; 64]);

impl fmt::Debug for Digest {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		let digest_string = self.0.map(|byte| format!("{:02x}", byte)).join(" ");
		formatter.write_str(&digest_string)
	}
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct Entry {
	hash: u64,
	#[br(pad_after = 4)]
	file_metadata: FileMetadata,
	// padding: u32,
}

impl Entry {
	const SIZE: u32 = 16;
}

#[binread]
#[derive(Clone, Debug)]
#[br(map = Self::read)]
pub struct FileMetadata {
	is_synonym: bool,
	pub data_file_id: u8,
	pub offset: u32,
}

impl FileMetadata {
	fn read(input: u32) -> Self {
		Self {
			is_synonym: (input & 0b1) == 0b1,
			data_file_id: ((input & 0b1110) >> 1) as u8,
			offset: (input & !0xF) * 0x08,
		}
	}
}
