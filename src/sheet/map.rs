use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Map {
    fn name() -> String {
        "Map".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Map::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Map {
    pub r#map_condition: u8,
    pub r#priority_category_ui: u8,
    pub r#priority_ui: u8,
    pub r#map_index: i8,
    pub r#hierarchy: u8,
    pub r#map_marker_range: u16,
    pub r#id: SeString,
    pub r#size_factor: u16,
    pub r#offset_x: i16,
    pub r#offset_y: i16,
    pub r#place_name_region: u16,
    pub r#place_name: u16,
    pub r#place_name_sub: u16,
    pub r#discovery_index: u8,
    pub r#discovery_flag: i16,
    pub r#territory_type: u32,
    pub r#discovery_array_byte: u16,
    pub r#is_event: bool,
    pub r#unknown18: bool,
    pub r#unknown19: bool,
    pub r#unknown20: u8,
}
impl Map {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#map_condition: row.field(0usize + offset)?.into_u8()?,
            r#priority_category_ui: row.field(1usize + offset)?.into_u8()?,
            r#priority_ui: row.field(2usize + offset)?.into_u8()?,
            r#map_index: row.field(3usize + offset)?.into_i8()?,
            r#hierarchy: row.field(4usize + offset)?.into_u8()?,
            r#map_marker_range: row.field(5usize + offset)?.into_u16()?,
            r#id: row.field(6usize + offset)?.into_string()?,
            r#size_factor: row.field(7usize + offset)?.into_u16()?,
            r#offset_x: row.field(8usize + offset)?.into_i16()?,
            r#offset_y: row.field(9usize + offset)?.into_i16()?,
            r#place_name_region: row.field(10usize + offset)?.into_u16()?,
            r#place_name: row.field(11usize + offset)?.into_u16()?,
            r#place_name_sub: row.field(12usize + offset)?.into_u16()?,
            r#discovery_index: row.field(13usize + offset)?.into_u8()?,
            r#discovery_flag: row.field(14usize + offset)?.into_i16()?,
            r#territory_type: row.field(15usize + offset)?.into_u32()?,
            r#discovery_array_byte: row.field(16usize + offset)?.into_u16()?,
            r#is_event: row.field(17usize + offset)?.into_bool()?,
            r#unknown18: row.field(18usize + offset)?.into_bool()?,
            r#unknown19: row.field(19usize + offset)?.into_bool()?,
            r#unknown20: row.field(20usize + offset)?.into_u8()?,
        })
    }
}
