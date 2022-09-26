use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for TreasureSpot {
    fn name() -> String {
        "TreasureSpot".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TreasureSpot::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TreasureSpot {
    pub r#location: i32,
    pub r#map_offset_x: f32,
    pub r#map_offset_y: f32,
}
impl TreasureSpot {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#location: row.field(0usize + offset)?.into_i32()?,
            r#map_offset_x: row.field(1usize + offset)?.into_f32()?,
            r#map_offset_y: row.field(2usize + offset)?.into_f32()?,
        })
    }
}
