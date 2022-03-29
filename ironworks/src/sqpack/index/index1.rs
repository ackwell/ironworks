use std::io::SeekFrom;

use binrw::BinRead;

use super::shared::{IndexHeader, SqPackHeader};

#[derive(BinRead, Debug)]
#[br(little)]
pub struct Index1 {
	sqpack_header: SqPackHeader,

	#[br(seek_before = SeekFrom::Start(sqpack_header.size.into()))]
	index_header: IndexHeader,
}
