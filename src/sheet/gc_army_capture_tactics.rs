use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for GcArmyCaptureTactics {
    fn name() -> String {
        "GcArmyCaptureTactics".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GcArmyCaptureTactics::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GcArmyCaptureTactics {
    pub r#name: i32,
    pub r#hp: u8,
    pub r#damage_dealt: u8,
    pub r#damage_received: u8,
    pub r#tactic: u32,
    pub r#icon: u32,
}
impl GcArmyCaptureTactics {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_i32()?,
            r#hp: row.field(1usize + offset)?.into_u8()?,
            r#damage_dealt: row.field(2usize + offset)?.into_u8()?,
            r#damage_received: row.field(3usize + offset)?.into_u8()?,
            r#tactic: row.field(4usize + offset)?.into_u32()?,
            r#icon: row.field(5usize + offset)?.into_u32()?,
        })
    }
}
