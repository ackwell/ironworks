use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for CraftAction {
    fn name() -> String {
        "CraftAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CraftAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CraftAction {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#animation_start: u16,
    pub r#animation_end: u16,
    pub r#icon: u16,
    pub r#class_job: i8,
    pub r#class_job_category: u8,
    pub r#class_job_level: u8,
    pub r#quest_requirement: u32,
    pub r#specialist: bool,
    pub r#unknown10: u16,
    pub r#cost: u8,
    pub r#crp: i32,
    pub r#bsm: i32,
    pub r#arm: i32,
    pub r#gsm: i32,
    pub r#ltw: i32,
    pub r#wvr: i32,
    pub r#alc: i32,
    pub r#cul: i32,
}
impl CraftAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#animation_start: row.field(2usize + offset)?.into_u16()?,
            r#animation_end: row.field(3usize + offset)?.into_u16()?,
            r#icon: row.field(4usize + offset)?.into_u16()?,
            r#class_job: row.field(5usize + offset)?.into_i8()?,
            r#class_job_category: row.field(6usize + offset)?.into_u8()?,
            r#class_job_level: row.field(7usize + offset)?.into_u8()?,
            r#quest_requirement: row.field(8usize + offset)?.into_u32()?,
            r#specialist: row.field(9usize + offset)?.into_bool()?,
            r#unknown10: row.field(10usize + offset)?.into_u16()?,
            r#cost: row.field(11usize + offset)?.into_u8()?,
            r#crp: row.field(12usize + offset)?.into_i32()?,
            r#bsm: row.field(13usize + offset)?.into_i32()?,
            r#arm: row.field(14usize + offset)?.into_i32()?,
            r#gsm: row.field(15usize + offset)?.into_i32()?,
            r#ltw: row.field(16usize + offset)?.into_i32()?,
            r#wvr: row.field(17usize + offset)?.into_i32()?,
            r#alc: row.field(18usize + offset)?.into_i32()?,
            r#cul: row.field(19usize + offset)?.into_i32()?,
        })
    }
}
