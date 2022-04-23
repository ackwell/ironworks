use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for RecipeLevelTable {
    fn name() -> String {
        "RecipeLevelTable".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RecipeLevelTable::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RecipeLevelTable {
    pub r#class_job_level: u8,
    pub r#stars: u8,
    pub r#suggested_craftsmanship: u16,
    pub r#suggested_control: u16,
    pub r#difficulty: u16,
    pub r#quality: u32,
    pub r#progress_divider: u8,
    pub r#quality_divider: u8,
    pub r#progress_modifier: u8,
    pub r#quality_modifier: u8,
    pub r#durability: u16,
    pub r#conditions_flag: u16,
}
impl RecipeLevelTable {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#class_job_level: row.field(0usize + offset)?.into_u8()?,
            r#stars: row.field(1usize + offset)?.into_u8()?,
            r#suggested_craftsmanship: row.field(2usize + offset)?.into_u16()?,
            r#suggested_control: row.field(3usize + offset)?.into_u16()?,
            r#difficulty: row.field(4usize + offset)?.into_u16()?,
            r#quality: row.field(5usize + offset)?.into_u32()?,
            r#progress_divider: row.field(6usize + offset)?.into_u8()?,
            r#quality_divider: row.field(7usize + offset)?.into_u8()?,
            r#progress_modifier: row.field(8usize + offset)?.into_u8()?,
            r#quality_modifier: row.field(9usize + offset)?.into_u8()?,
            r#durability: row.field(10usize + offset)?.into_u16()?,
            r#conditions_flag: row.field(11usize + offset)?.into_u16()?,
        })
    }
}
