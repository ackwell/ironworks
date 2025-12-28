use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for RetainerTaskNormal {
    fn name() -> String {
        "RetainerTaskNormal".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RetainerTaskNormal::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RetainerTaskNormal {
    pub r#item: i32,
    pub r#quantity: Vec<u8>,
    pub r#gathering_log: i16,
    pub r#fishing_log: i16,
}
impl RetainerTaskNormal {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#quantity: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u8()?) },
            )?,
            r#gathering_log: row.field(6usize + offset)?.into_i16()?,
            r#fishing_log: row.field(7usize + offset)?.into_i16()?,
        })
    }
}
