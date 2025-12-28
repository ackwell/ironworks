use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for FishingNoteInfo {
    fn name() -> String {
        "FishingNoteInfo".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FishingNoteInfo::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FishingNoteInfo {
    pub r#size: u8,
    pub r#aquarium_water: u8,
    pub r#weather_restriction: u8,
    pub r#time_restriction: u8,
    pub r#special_conditions: u8,
    pub r#is_collectable: u8,
    pub r#item: i32,
}
impl FishingNoteInfo {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#size: row.field(0usize + offset)?.into_u8()?,
            r#aquarium_water: row.field(1usize + offset)?.into_u8()?,
            r#weather_restriction: row.field(2usize + offset)?.into_u8()?,
            r#time_restriction: row.field(3usize + offset)?.into_u8()?,
            r#special_conditions: row.field(4usize + offset)?.into_u8()?,
            r#is_collectable: row.field(5usize + offset)?.into_u8()?,
            r#item: row.field(6usize + offset)?.into_i32()?,
        })
    }
}
