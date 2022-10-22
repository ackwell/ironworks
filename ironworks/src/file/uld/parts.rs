use binrw::binread;

use super::shared::ByteString;

#[binread]
#[br(little)]
#[br(import(magic: ByteString<4>, _version: ByteString<4>))]
#[br(pre_assert(
	&magic == b"tphd",
	"incorrect magic, expected b\"tphd\", got {:?}",
	magic
))]
#[derive(Debug)]
pub struct Parts {
	id: u32,
	#[br(temp)]
	count: u32,
	size: u32,
	#[br(count = count)]
	parts: Vec<Part>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Part {
	texture_id: u32,
	u: u16,
	v: u16,
	width: u16,
	height: u16,
}
