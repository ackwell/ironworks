use std::{collections::BTreeSet, io::SeekFrom};

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
	hash: u32,
	file_metadata: FileMetadata,
}

impl Entry {
	const SIZE: u32 = 8;
}

#[binread]
#[derive(Debug)]
#[br(little)]
pub struct Index2 {
	#[br(temp)]
	sqpack_header: SqPackHeader,

	#[br(temp, seek_before = SeekFrom::Start(sqpack_header.size.into()))]
	index_header: IndexHeader,

	#[br(
    seek_before = SeekFrom::Start(index_header.index_data.offset.into()),
    count = index_header.index_data.size / Entry::SIZE,
  )]
	indexes: Vec<Entry>,

	#[br(calc = indexes.iter().map(|entry| (
		entry.file_metadata.data_file_id,
		entry.file_metadata.offset
	)).collect())]
	offsets: BTreeSet<(u8, u64)>,
}

impl Index2 {
	// TODO: this is almost purely duplicated with index1 - dedupe somehow?
	pub fn find(&self, path: &str) -> Result<(FileMetadata, Option<u64>)> {
		let hash = crc32(path.as_bytes());

		self.indexes
			.iter()
			.find(|entry| entry.hash == hash)
			.map(|entry| {
				let metadata = entry.file_metadata.clone();

				let size = self
					.offsets
					.range((metadata.data_file_id, metadata.offset + 1)..)
					.next()
					.and_then(|(dat_id, offset)| match *dat_id == metadata.data_file_id {
						true => Some(offset - metadata.offset),
						false => None,
					});

				(metadata, size)
			})
			.ok_or_else(|| Error::NotFound(ErrorValue::Path(path.into())))
	}
}
