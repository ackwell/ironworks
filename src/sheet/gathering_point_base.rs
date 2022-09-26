use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::utility::read_array;
use std::result::Result;
use std::vec::Vec;
use crate::error::PopulateError;
impl MetadataAdapter for GatheringPointBase {
    fn name() -> String {
        "GatheringPointBase".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringPointBase::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringPointBase {
    pub r#gathering_type: i32,
    pub r#gathering_level: u8,
    pub r#item: Vec<i32>,
}
impl GatheringPointBase {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#gathering_type: row.field(0usize + offset)?.into_i32()?,
            r#gathering_level: row.field(1usize + offset)?.into_u8()?,
            r#item: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_i32()?) },
            )?,
        })
    }
}
