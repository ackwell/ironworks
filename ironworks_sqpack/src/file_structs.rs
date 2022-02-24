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
	header: SharedHeader,
	#[br(seek_before = SeekFrom::Start(header.size.into()))]
	index_header: IndexHeader,
}

// TODO: etc
// TODO: name? FileHeader or
#[derive(Debug)]
#[binread]
#[br(little, magic = b"SqPack\0\0")]
pub struct SharedHeader {
	platform_id: u8,
	#[br(pad_before = 3)] // unknown1
	size: u32,
	version: u32,
	type_: u32,
}

// TODO: there's actually a lot more to this, check lumina/kobold impls.
#[derive(Debug)]
#[binread]
#[br(little)]
pub struct IndexHeader {
	size: u32,
	version: u32,
	index_data: Metadata,
	data_file_count: u32,
	synonym_data: Metadata,
	empty_block_data: Metadata,
	dir_index_data: Metadata,
	index_type: u32,
	#[br(pad_before = 656)] // reserved
	digest: Digest,
}

#[derive(Debug)]
#[binread]
#[br(little)]
pub struct Metadata {
	offset: u32,
	size: u32,
	digest: Digest,
}

#[binread]
struct Digest([u8; 64]);

impl Debug for Digest {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		let digest_string = self.0.map(|byte| format!("{:02x}", byte)).join(" ");
		return formatter.write_str(&digest_string);
	}
}
