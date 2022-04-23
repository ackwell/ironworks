use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for TerritoryType {
    fn name() -> String {
        "TerritoryType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TerritoryType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TerritoryType {
    pub r#name: SeString,
    pub r#bg: SeString,
    pub r#battalion_mode: u8,
    pub r#place_name_region: u16,
    pub r#place_name_zone: u16,
    pub r#place_name: u16,
    pub r#map: u16,
    pub r#loading_image: u8,
    pub r#exclusive_type: u8,
    pub r#territory_intended_use: u8,
    pub r#content_finder_condition: u16,
    pub r#unknown11: bool,
    pub r#weather_rate: u8,
    pub r#unknown13: bool,
    pub r#unknown14: u8,
    pub r#pc_search: bool,
    pub r#stealth: bool,
    pub r#mount: bool,
    pub r#unknown18: bool,
    pub r#bgm: u16,
    pub r#place_name_region_icon: i32,
    pub r#place_name_icon: i32,
    pub r#array_event_handler: u32,
    pub r#quest_battle: u16,
    pub r#aetheryte: i32,
    pub r#fixed_time: i32,
    pub r#resident: u16,
    pub r#achievement_index: i8,
    pub r#is_pvp_zone: bool,
    pub r#ex_version: u8,
    pub r#unknown30: u8,
    pub r#unknown31: u8,
    pub r#unknown32: u8,
    pub r#mount_speed: u8,
}
impl TerritoryType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#bg: row.field(1usize + offset)?.into_string()?,
            r#battalion_mode: row.field(2usize + offset)?.into_u8()?,
            r#place_name_region: row.field(3usize + offset)?.into_u16()?,
            r#place_name_zone: row.field(4usize + offset)?.into_u16()?,
            r#place_name: row.field(5usize + offset)?.into_u16()?,
            r#map: row.field(6usize + offset)?.into_u16()?,
            r#loading_image: row.field(7usize + offset)?.into_u8()?,
            r#exclusive_type: row.field(8usize + offset)?.into_u8()?,
            r#territory_intended_use: row.field(9usize + offset)?.into_u8()?,
            r#content_finder_condition: row.field(10usize + offset)?.into_u16()?,
            r#unknown11: row.field(11usize + offset)?.into_bool()?,
            r#weather_rate: row.field(12usize + offset)?.into_u8()?,
            r#unknown13: row.field(13usize + offset)?.into_bool()?,
            r#unknown14: row.field(14usize + offset)?.into_u8()?,
            r#pc_search: row.field(15usize + offset)?.into_bool()?,
            r#stealth: row.field(16usize + offset)?.into_bool()?,
            r#mount: row.field(17usize + offset)?.into_bool()?,
            r#unknown18: row.field(18usize + offset)?.into_bool()?,
            r#bgm: row.field(19usize + offset)?.into_u16()?,
            r#place_name_region_icon: row.field(20usize + offset)?.into_i32()?,
            r#place_name_icon: row.field(21usize + offset)?.into_i32()?,
            r#array_event_handler: row.field(22usize + offset)?.into_u32()?,
            r#quest_battle: row.field(23usize + offset)?.into_u16()?,
            r#aetheryte: row.field(24usize + offset)?.into_i32()?,
            r#fixed_time: row.field(25usize + offset)?.into_i32()?,
            r#resident: row.field(26usize + offset)?.into_u16()?,
            r#achievement_index: row.field(27usize + offset)?.into_i8()?,
            r#is_pvp_zone: row.field(28usize + offset)?.into_bool()?,
            r#ex_version: row.field(29usize + offset)?.into_u8()?,
            r#unknown30: row.field(30usize + offset)?.into_u8()?,
            r#unknown31: row.field(31usize + offset)?.into_u8()?,
            r#unknown32: row.field(32usize + offset)?.into_u8()?,
            r#mount_speed: row.field(33usize + offset)?.into_u8()?,
        })
    }
}
