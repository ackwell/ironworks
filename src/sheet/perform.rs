use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Perform {
    fn name() -> String {
        "Perform".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Perform::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Perform {
    pub r#name: SeString,
    pub r#unknown1: bool,
    pub r#model_key: u64,
    pub r#animation_start: u16,
    pub r#animation_end: u16,
    pub r#animation_idle: u16,
    pub r#animation_play01: u16,
    pub r#animation_play02: u16,
    pub r#stop_animation: i32,
    pub r#instrument: SeString,
    pub r#order: i32,
    pub r#transient: u8,
    pub r#unknown12: u8,
}
impl Perform {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#model_key: row.field(2usize + offset)?.into_u64()?,
            r#animation_start: row.field(3usize + offset)?.into_u16()?,
            r#animation_end: row.field(4usize + offset)?.into_u16()?,
            r#animation_idle: row.field(5usize + offset)?.into_u16()?,
            r#animation_play01: row.field(6usize + offset)?.into_u16()?,
            r#animation_play02: row.field(7usize + offset)?.into_u16()?,
            r#stop_animation: row.field(8usize + offset)?.into_i32()?,
            r#instrument: row.field(9usize + offset)?.into_string()?,
            r#order: row.field(10usize + offset)?.into_i32()?,
            r#transient: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
        })
    }
}
