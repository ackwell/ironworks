use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for DynamicEvent {
    fn name() -> String {
        "DynamicEvent".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DynamicEvent::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DynamicEvent {
    pub r#event_type: u8,
    pub r#enemy_type: u8,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#lgb_event_object: bool,
    pub r#lgb_map_range: u32,
    pub r#quest: u32,
    pub r#unknown7: u32,
    pub r#single_battle: u8,
    pub r#announce: u8,
    pub r#name: u32,
    pub r#description: SeString,
    pub r#unknown12: SeString,
    pub r#unknown13: i16,
    pub r#unknown14: i16,
    pub r#unknown15: i16,
    pub r#unknown16: u32,
    pub r#unknown17: u32,
}
impl DynamicEvent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#event_type: row.field(0usize + offset)?.into_u8()?,
            r#enemy_type: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#lgb_event_object: row.field(4usize + offset)?.into_bool()?,
            r#lgb_map_range: row.field(5usize + offset)?.into_u32()?,
            r#quest: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
            r#single_battle: row.field(8usize + offset)?.into_u8()?,
            r#announce: row.field(9usize + offset)?.into_u8()?,
            r#name: row.field(10usize + offset)?.into_u32()?,
            r#description: row.field(11usize + offset)?.into_string()?,
            r#unknown12: row.field(12usize + offset)?.into_string()?,
            r#unknown13: row.field(13usize + offset)?.into_i16()?,
            r#unknown14: row.field(14usize + offset)?.into_i16()?,
            r#unknown15: row.field(15usize + offset)?.into_i16()?,
            r#unknown16: row.field(16usize + offset)?.into_u32()?,
            r#unknown17: row.field(17usize + offset)?.into_u32()?,
        })
    }
}
