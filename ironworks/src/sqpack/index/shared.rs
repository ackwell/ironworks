use std::fmt;

use binrw::BinRead;

#[derive(BinRead, Debug)]
#[br(little, magic = b"SqPack\0\0")]
pub struct SqPackHeader {
	platform_id: PlatformId,
	// unknown: [u8; 3],
	#[br(pad_before = 3)]
	pub size: u32,
	version: u32,
	kind: u32,
}

#[derive(BinRead, Debug)]
#[br(repr = u8)]
enum PlatformId {
	Win32,
	PS3,
	PS4,
}

#[derive(BinRead, Debug)]
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

#[derive(BinRead, Debug)]
#[br(little)]
pub struct Metadata {
	offset: u32,
	size: u32,
	digest: Digest,
}

#[derive(BinRead)]
struct Digest([u8; 64]);

impl fmt::Debug for Digest {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		let digest_string = self.0.map(|byte| format!("{:02x}", byte)).join(" ");
		formatter.write_str(&digest_string)
	}
}
