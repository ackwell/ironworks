use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MapMarker {
    fn name() -> String {
        "MapMarker".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MapMarker::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MapMarker {
    pub r#x: i16,
    pub r#y: i16,
    pub r#icon: u16,
    pub r#place_name_subtext: u16,
    pub r#subtext_orientation: u8,
    pub r#map_marker_region: u8,
    pub r#type: u8,
    pub r#data_type: u8,
    pub r#data_key: u16,
    pub r#unknown9: u8,
    pub r#unknown10: u16,
}
impl MapMarker {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#x: row.field(0usize + offset)?.into_i16()?,
            r#y: row.field(1usize + offset)?.into_i16()?,
            r#icon: row.field(2usize + offset)?.into_u16()?,
            r#place_name_subtext: row.field(3usize + offset)?.into_u16()?,
            r#subtext_orientation: row.field(4usize + offset)?.into_u8()?,
            r#map_marker_region: row.field(5usize + offset)?.into_u8()?,
            r#type: row.field(6usize + offset)?.into_u8()?,
            r#data_type: row.field(7usize + offset)?.into_u8()?,
            r#data_key: row.field(8usize + offset)?.into_u16()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#unknown10: row.field(10usize + offset)?.into_u16()?,
        })
    }
}
