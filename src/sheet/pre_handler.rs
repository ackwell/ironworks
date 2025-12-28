use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for PreHandler {
    fn name() -> String {
        "PreHandler".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PreHandler::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PreHandler {
    pub r#unknown0: SeString,
    pub r#image: u32,
    pub r#target: u32,
    pub r#unlock_quest: u32,
    pub r#accept_message: u32,
    pub r#deny_message: u32,
    pub r#unknown6: u8,
    pub r#unknown7: u8,
}
impl PreHandler {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_string()?,
            r#image: row.field(1usize + offset)?.into_u32()?,
            r#target: row.field(2usize + offset)?.into_u32()?,
            r#unlock_quest: row.field(3usize + offset)?.into_u32()?,
            r#accept_message: row.field(4usize + offset)?.into_u32()?,
            r#deny_message: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
        })
    }
}
