use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for Level {
    fn name() -> String {
        "Level".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Level::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Level {
    pub r#x: f32,
    pub r#y: f32,
    pub r#z: f32,
    pub r#yaw: f32,
    pub r#radius: f32,
    pub r#type: u8,
    pub r#object: u32,
    pub r#map: u16,
    pub r#event_id: u32,
    pub r#territory: u16,
}
impl Level {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#x: row.field(0usize + offset)?.into_f32()?,
            r#y: row.field(1usize + offset)?.into_f32()?,
            r#z: row.field(2usize + offset)?.into_f32()?,
            r#yaw: row.field(3usize + offset)?.into_f32()?,
            r#radius: row.field(4usize + offset)?.into_f32()?,
            r#type: row.field(5usize + offset)?.into_u8()?,
            r#object: row.field(6usize + offset)?.into_u32()?,
            r#map: row.field(7usize + offset)?.into_u16()?,
            r#event_id: row.field(8usize + offset)?.into_u32()?,
            r#territory: row.field(9usize + offset)?.into_u16()?,
        })
    }
}
