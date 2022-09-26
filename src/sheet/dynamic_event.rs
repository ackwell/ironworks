use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
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
    pub r#lgb_event_object: u32,
    pub r#lgb_map_range: u32,
    pub r#quest: u32,
    pub r#unknown7: u8,
    pub r#single_battle: u8,
    pub r#announce: u32,
    pub r#name: SeString,
    pub r#description: SeString,
}
impl DynamicEvent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#event_type: row.field(0usize + offset)?.into_u8()?,
            r#enemy_type: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#lgb_event_object: row.field(4usize + offset)?.into_u32()?,
            r#lgb_map_range: row.field(5usize + offset)?.into_u32()?,
            r#quest: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
            r#single_battle: row.field(8usize + offset)?.into_u8()?,
            r#announce: row.field(9usize + offset)?.into_u32()?,
            r#name: row.field(10usize + offset)?.into_string()?,
            r#description: row.field(11usize + offset)?.into_string()?,
        })
    }
}
