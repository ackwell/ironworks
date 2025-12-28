use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for CSBonusSeason {
    fn name() -> String {
        "CSBonusSeason".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CSBonusSeason::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CSBonusSeason {
    pub r#unknown0: bool,
    pub r#unknown1: u16,
    pub r#unknown2: u16,
    pub r#unknown3: bool,
    pub r#unknown4: bool,
    pub r#item: u32,
    pub r#category: Vec<u16>,
    pub r#text: Vec<u8>,
    pub r#unknown12: u8,
}
impl CSBonusSeason {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#item: row.field(5usize + offset)?.into_u32()?,
            r#category: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(6usize + offset)?.into_u16()?) },
            )?,
            r#text: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(10usize + offset)?.into_u8()?) },
            )?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
        })
    }
}
