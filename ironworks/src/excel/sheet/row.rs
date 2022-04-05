use binrw::binread;

#[binread]
#[derive(Debug)]
#[br(big)]
pub struct RowHeader {
	pub data_size: u32,
	pub row_count: u16,
}

#[binread]
#[derive(Debug)]
#[br(big)]
pub struct SubrowHeader {
	subrow_id: u16,
}

impl SubrowHeader {
	pub const SIZE: u16 = 2;
}
