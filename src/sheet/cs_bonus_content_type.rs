use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for CSBonusContentType {
    fn name() -> String {
        "CSBonusContentType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CSBonusContentType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CSBonusContentType {
    pub r#content_type: u8,
    pub r#text: Vec<u32>,
    pub r#image: u32,
    pub r#unknown6: u32,
    pub r#unlock_quest: bool,
    pub r#unknown8: u32,
    pub r#unknown9: u32,
    pub r#unknown10: u32,
}
impl CSBonusContentType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#content_type: row.field(0usize + offset)?.into_u8()?,
            r#text: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u32()?) },
            )?,
            r#image: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#unlock_quest: row.field(7usize + offset)?.into_bool()?,
            r#unknown8: row.field(8usize + offset)?.into_u32()?,
            r#unknown9: row.field(9usize + offset)?.into_u32()?,
            r#unknown10: row.field(10usize + offset)?.into_u32()?,
        })
    }
}
