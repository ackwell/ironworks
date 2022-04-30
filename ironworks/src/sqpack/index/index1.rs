use std::io::SeekFrom;

use binrw::binread;

use crate::error::{Error, ErrorValue, Result};

use super::{
	crc::crc32,
	shared::{FileMetadata, IndexHeader, SqPackHeader},
};

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
#[derive(Debug)]
#[br(little)]
pub struct Index1 {
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

impl Index1 {
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
