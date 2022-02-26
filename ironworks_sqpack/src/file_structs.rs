// We don't need everything we read
// TODO: Consider this. Might be overhead we can avoid?
#![allow(dead_code)]

use binrw::binread;
use modular_bitfield::{bitfield, prelude::*};
use std::{
	fmt::{self, Debug},
	io::SeekFrom,
};

#[derive(Debug)]
#[binread]
#[br(little)]
pub struct Index {
	pub header: SharedHeader,

	#[br(seek_before = SeekFrom::Start(header.size.into()))]
	pub index_header: IndexHeader,

	#[br(
		seek_before = SeekFrom::Start(index_header.index_data.offset.into()),
		count = index_header.index_data.size / INDEX_HASH_TABLE_ENTRY_SIZE,
	)]
	pub indexes: Vec<IndexHashTableEntry>,
}

#[derive(Debug)]
#[binread]
#[br(little, magic = b"SqPack\0\0")]
pub struct SharedHeader {
	pub platform_id: u8,

	#[br(pad_before = 3)] // unknown1
	pub size: u32,
	pub version: u32,
	pub type_: u32,
}

#[derive(Debug)]
#[binread]
#[br(little)]
pub struct IndexHeader {
	pub size: u32,
	pub version: u32,
	pub index_data: Metadata,
	pub data_file_count: u32,
	pub synonym_data: Metadata,
	pub empty_block_data: Metadata,
	pub dir_index_data: Metadata,
	pub index_type: u32,

	#[br(pad_before = 656)] // reserved
	pub digest: Digest,
}

#[derive(Debug)]
#[binread]
#[br(little)]
pub struct Metadata {
	pub offset: u32,
	pub size: u32,
	pub digest: Digest,
}

#[binread]
pub struct Digest([u8; 64]);

impl Debug for Digest {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		let digest_string = self.0.map(|byte| format!("{:02x}", byte)).join(" ");
		return formatter.write_str(&digest_string);
	}
}

const INDEX_HASH_TABLE_ENTRY_SIZE: u32 = 16;

#[derive(Debug)]
#[binread]
#[br(little)]
pub struct IndexHashTableEntry {
	pub hash: u64,

	#[br(pad_after = 4)] // padding
	pub value: IndexHashTableValue,
}

#[bitfield]
#[binread]
#[derive(Debug)]
#[br(little, map = Self::from_bytes)]
pub struct IndexHashTableValue {
	pub is_synonym: bool,
	pub data_file_id: B3,
	pub offset: B28,
}
