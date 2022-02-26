// We don't need everything we read
// TODO: Consider this. Might be overhead we can avoid?
#![allow(dead_code)]

use binrw::binread;
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
		count = index_header.index_data.size / IndexHashTableEntry::SIZE as u32,
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

#[derive(Debug)]
#[binread]
#[br(little)]
pub struct IndexHashTableEntry {
	pub hash: u64,

	#[br(pad_after = 4)] // padding
	pub value: IndexHashTableValue,
}

impl IndexHashTableEntry {
	pub const SIZE: usize = 16;
}

#[derive(Debug)]
#[binread]
#[br(map = Self::read)]
pub struct IndexHashTableValue {
	pub is_synonym: bool,
	pub data_file_id: u8,
	pub offset: u32,
}

impl IndexHashTableValue {
	fn read(input: u32) -> Self {
		return IndexHashTableValue {
			is_synonym: (input & 0b1) == 0b1,
			data_file_id: ((input & 0b1110) >> 1) as u8,
			offset: (input & !0xF) * 0x08,
		};
	}
}

// TODO: different file? maybe?

#[derive(Debug)]
#[binread]
#[br(little)]
pub struct FileInfo {
	pub size: u32,
	pub type_: FileType,
	pub raw_file_size: u32,
	#[br(pad_before = 8)]
	pub block_count: u32,
}

impl FileInfo {
	pub const SIZE: usize = 24;
}

#[derive(Debug)]
#[binread]
#[br(little, repr = u32)]
pub enum FileType {
	Empty = 1,
	Standard = 2,
	Model = 3,
	Texture = 4,
}

// TODO: i think this can be moved into the file info with some compute? maybe not

#[derive(Debug)]
#[binread]
#[br(little)]
pub struct BlockInfo {
	pub offset: u32,
	pub size: u16,
	pub uncompressed_size: u16,
}

impl BlockInfo {
	pub const SIZE: usize = 8;
}

#[derive(Debug)]
#[binread]
#[br(little)]
pub struct BlockHeader {
	pub size: u32,
	#[br(pad_before = 4)] // unknown1
	pub compressed_size: u32,
	pub uncompressed_size: u32,
}
