use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for MJIRank {
    fn name() -> String {
        "MJIRank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIRank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIRank {
    pub r#exp_to_next: u32,
    pub r#unknown1: u8,
    pub r#log_message: Vec<u32>,
}
impl MJIRank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#exp_to_next: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#log_message: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
