use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for CSBonusContentIdentifier {
    fn name() -> String {
        "CSBonusContentIdentifier".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CSBonusContentIdentifier::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CSBonusContentIdentifier {
    pub r#content_link_type: u8,
    pub r#content: u32,
    pub r#unknown2: bool,
    pub r#unlock_quest: Vec<u32>,
    pub r#unknown6: u32,
    pub r#map: u32,
}
impl CSBonusContentIdentifier {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#content_link_type: row.field(0usize + offset)?.into_u8()?,
            r#content: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unlock_quest: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_u32()?) },
            )?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#map: row.field(7usize + offset)?.into_u32()?,
        })
    }
}
