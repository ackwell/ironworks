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
	pub id: u16,
}

impl SubrowHeader {
	pub const SIZE: u16 = 2;
}

/// A (sub)row within an Excel sheet.
#[derive(Debug)]
pub struct Row {
	// TODO: do we make these public or use fns
	row_id: u32,
	subrow_id: u16,

}

impl Row {
	pub(super) fn new(row_id: u32, subrow_id: u16) -> Self {
		Self {
			row_id,
			subrow_id,
		}
	}
}
