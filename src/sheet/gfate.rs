use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::utility::read_array;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for GFATE {
    fn name() -> String {
        "GFATE".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GFATE::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GFATE {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#unknown2: u32,
    pub r#unknown3: u16,
    pub r#unknown4: u16,
    pub r#unknown5: u16,
    pub r#unknown6: u32,
    pub r#lgb_pop_range: Vec<u32>,
    pub r#unknown22: u32,
    pub r#icon: Vec<u32>,
}
impl GFATE {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u32()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u16()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#lgb_pop_range: read_array(
                offset,
                15usize,
                1usize,
                |offset| { Result::Ok(row.field(7usize + offset)?.into_u32()?) },
            )?,
            r#unknown22: row.field(22usize + offset)?.into_u32()?,
            r#icon: read_array(
                offset,
                15usize,
                1usize,
                |offset| { Result::Ok(row.field(23usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
