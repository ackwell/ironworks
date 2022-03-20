use binrw::{BinRead, NullString};

#[derive(BinRead, Debug)]
pub struct SeString {
	// TODO: Probably should store this as a byte vec with utils
	#[allow(dead_code)]
	raw: NullString,
}
