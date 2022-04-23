use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for Achievement {
    fn name() -> String {
        "Achievement".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Achievement::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Achievement {
    pub r#achievement_category: u8,
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#achievement_target: u8,
    pub r#unknown4: u8,
    pub r#points: u8,
    pub r#title: u16,
    pub r#item: u32,
    pub r#unknown8: u8,
    pub r#unknown9: u8,
    pub r#unknown10: u8,
    pub r#icon: u16,
    pub r#unknown12: u8,
    pub r#type: u8,
    pub r#key: i32,
    pub r#data: Vec<i32>,
    pub r#order: u16,
    pub r#unknown24: u8,
    pub r#achievement_hide_condition: u8,
}
impl Achievement {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#achievement_category: row.field(0usize + offset)?.into_u8()?,
            r#name: row.field(1usize + offset)?.into_string()?,
            r#description: row.field(2usize + offset)?.into_string()?,
            r#achievement_target: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
            r#points: row.field(5usize + offset)?.into_u8()?,
            r#title: row.field(6usize + offset)?.into_u16()?,
            r#item: row.field(7usize + offset)?.into_u32()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#unknown10: row.field(10usize + offset)?.into_u8()?,
            r#icon: row.field(11usize + offset)?.into_u16()?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
            r#type: row.field(13usize + offset)?.into_u8()?,
            r#key: row.field(14usize + offset)?.into_i32()?,
            r#data: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(15usize + offset)?.into_i32()?) },
            )?,
            r#order: row.field(23usize + offset)?.into_u16()?,
            r#unknown24: row.field(24usize + offset)?.into_u8()?,
            r#achievement_hide_condition: row.field(25usize + offset)?.into_u8()?,
        })
    }
}
