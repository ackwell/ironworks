use ironworks::sestring::SeString;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Adventure {
    fn name() -> String {
        "Adventure".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Adventure::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Adventure {
    pub r#level: i32,
    pub r#min_level: i32,
    pub r#max_level: u8,
    pub r#emote: u16,
    pub r#min_time: u16,
    pub r#max_time: u16,
    pub r#place_name: i32,
    pub r#icon_list: i32,
    pub r#icon_discovered: i32,
    pub r#name: SeString,
    pub r#impression: SeString,
    pub r#description: SeString,
    pub r#icon_undiscovered: i32,
    pub r#is_initial: bool,
}
impl Adventure {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#level: row.field(0usize + offset)?.into_i32()?,
            r#min_level: row.field(1usize + offset)?.into_i32()?,
            r#max_level: row.field(2usize + offset)?.into_u8()?,
            r#emote: row.field(3usize + offset)?.into_u16()?,
            r#min_time: row.field(4usize + offset)?.into_u16()?,
            r#max_time: row.field(5usize + offset)?.into_u16()?,
            r#place_name: row.field(6usize + offset)?.into_i32()?,
            r#icon_list: row.field(7usize + offset)?.into_i32()?,
            r#icon_discovered: row.field(8usize + offset)?.into_i32()?,
            r#name: row.field(9usize + offset)?.into_string()?,
            r#impression: row.field(10usize + offset)?.into_string()?,
            r#description: row.field(11usize + offset)?.into_string()?,
            r#icon_undiscovered: row.field(12usize + offset)?.into_i32()?,
            r#is_initial: row.field(13usize + offset)?.into_bool()?,
        })
    }
}
