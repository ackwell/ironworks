use std::{fmt, io::SeekFrom, rc::Rc};

use binrw::BinRead;

use crate::error::Result;

use super::Resource;

// do i just trait this?
// or would making it a trait make the reader, and then the sqpack, need to generic over it
// in that case wrapper makes more sense i guess

// tempted to say index owns chunks and then it can return file locations like the old one but with less wiring

// with the binary reading stuff this should probably be split up into a few files

#[derive(Debug)]
pub struct Index<R> {
	resource: Rc<R>,
}

impl<R: Resource> Index<R> {
	pub fn new(resource: Rc<R>) -> Result<Self> {
		// TODO: handle chunks

		// ergh does this mean we need to pass the meta down here too? this is getting messy.
		let mut foo = resource.index(0, 10, 0)?;
		let fsdf = Index1::read(&mut foo);
		println!("uuuuuh... something? {fsdf:#?}");

		Ok(Self { resource })
	}
}

#[derive(BinRead, Debug)]
#[br(little, magic = b"SqPack\0\0")]
struct SqPackHeader {
	platform_id: PlatformId,
	// unknown: [u8; 3],
	#[br(pad_before = 3)]
	size: u32,
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
struct IndexHeader {
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

#[derive(BinRead, Debug)]
#[br(little)]
struct Index1 {
	sqpack_header: SqPackHeader,

	#[br(seek_before = SeekFrom::Start(sqpack_header.size.into()))]
	index_header: IndexHeader,
}

// todo
struct Index2 {}
