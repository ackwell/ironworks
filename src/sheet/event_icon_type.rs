use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for EventIconType {
    fn name() -> String {
        "EventIconType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EventIconType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EventIconType {
    pub r#npc_icon_available: u32,
    pub r#map_icon_available: u32,
    pub r#npc_icon_invalid: u32,
    pub r#map_icon_invalid: u32,
    pub r#icon_range: u8,
}
impl EventIconType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#npc_icon_available: row.field(0usize + offset)?.into_u32()?,
            r#map_icon_available: row.field(1usize + offset)?.into_u32()?,
            r#npc_icon_invalid: row.field(2usize + offset)?.into_u32()?,
            r#map_icon_invalid: row.field(3usize + offset)?.into_u32()?,
            r#icon_range: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
