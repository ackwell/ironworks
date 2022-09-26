use std::result::Result;
use crate::error::PopulateError;
use crate::utility::read_array;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
impl MetadataAdapter for FishingSpot {
    fn name() -> String {
        "FishingSpot".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FishingSpot::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FishingSpot {
    pub r#gathering_level: u8,
    pub r#big_fish_on_reach: SeString,
    pub r#big_fish_on_end: SeString,
    pub r#fishing_spot_category: u8,
    pub r#rare: bool,
    pub r#territory_type: u16,
    pub r#place_name_main: u16,
    pub r#place_name_sub: u16,
    pub r#x: i16,
    pub r#z: i16,
    pub r#radius: u16,
    pub r#unknown11: u8,
    pub r#item: Vec<i32>,
    pub r#place_name: u16,
    pub r#order: u16,
}
impl FishingSpot {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#gathering_level: row.field(0usize + offset)?.into_u8()?,
            r#big_fish_on_reach: row.field(1usize + offset)?.into_string()?,
            r#big_fish_on_end: row.field(2usize + offset)?.into_string()?,
            r#fishing_spot_category: row.field(3usize + offset)?.into_u8()?,
            r#rare: row.field(4usize + offset)?.into_bool()?,
            r#territory_type: row.field(5usize + offset)?.into_u16()?,
            r#place_name_main: row.field(6usize + offset)?.into_u16()?,
            r#place_name_sub: row.field(7usize + offset)?.into_u16()?,
            r#x: row.field(8usize + offset)?.into_i16()?,
            r#z: row.field(9usize + offset)?.into_i16()?,
            r#radius: row.field(10usize + offset)?.into_u16()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
            r#item: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(row.field(12usize + offset)?.into_i32()?) },
            )?,
            r#place_name: row.field(22usize + offset)?.into_u16()?,
            r#order: row.field(23usize + offset)?.into_u16()?,
        })
    }
}
