use std::fmt;

use binrw::BinRead;

#[derive(BinRead, Debug)]
#[br(little, magic = b"SqPack\0\0")]
pub struct SqPackHeader {
	_platform_id: u8,
	// unknown: [u8; 3],
	#[br(pad_before = 3)]
	pub size: u32,
	_version: u32,
	_kind: u32,
}

#[derive(BinRead, Debug)]
#[br(little)]
pub struct IndexHeader {
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

#[derive(BinRead, Debug)]
#[br(little)]
pub struct Section {
	pub offset: u32,
	pub size: u32,
	_digest: Digest,
}

#[derive(BinRead)]
struct Digest([u8; 64]);

impl fmt::Debug for Digest {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		let digest_string = self.0.map(|byte| format!("{:02x}", byte)).join(" ");
		formatter.write_str(&digest_string)
	}
}

#[derive(BinRead, Clone, Debug)]
#[br(map = Self::read)]
pub struct FileMetadata {
	is_synonym: bool,
	pub data_file_id: u8,
	pub offset: u64,
}

impl FileMetadata {
	fn read(input: u32) -> Self {
		Self {
			is_synonym: (input & 0b1) == 0b1,
			data_file_id: ((input & 0b1110) >> 1) as u8,
			offset: (input as u64 & !0xF) * 0x08,
		}
	}
}
