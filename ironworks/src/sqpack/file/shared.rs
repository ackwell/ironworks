use binrw::binread;

#[binread]
#[derive(Debug)]
#[br(little)]
pub struct Header {
	pub size: u32,
	pub kind: FileKind,
	pub raw_file_size: u32,
	// num_blocks: u32,
	// block_buffer_size: u32,
	#[br(pad_before = 8)]
	pub block_count: u32,
}

#[binread]
#[derive(Debug)]
#[br(little, repr = u32)]
pub enum FileKind {
	Empty = 1,
	Standard,
	Model,
	Texture,
}
