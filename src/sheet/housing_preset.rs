use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
impl MetadataAdapter for HousingPreset {
    fn name() -> String {
        "HousingPreset".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HousingPreset::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HousingPreset {
    pub r#singular: SeString,
    pub r#adjective: i8,
    pub r#plural: SeString,
    pub r#possessive_pronoun: i8,
    pub r#starts_with_vowel: i8,
    pub r#unknown5: i8,
    pub r#pronoun: i8,
    pub r#article: i8,
    pub r#place_name: u16,
    pub r#housing_size: u8,
    pub r#exterior_roof: i32,
    pub r#exterior_wall: i32,
    pub r#exterior_window: i32,
    pub r#exterior_door: i32,
    pub r#interior_wall: i32,
    pub r#interior_flooring: i32,
    pub r#interior_lighting: i32,
    pub r#other_floor_wall: i32,
    pub r#other_floor_flooring: i32,
    pub r#other_floor_lighting: i32,
    pub r#basement_wall: i32,
    pub r#basement_flooring: i32,
    pub r#basement_lighting: i32,
    pub r#mansion_lighting: i32,
}
impl HousingPreset {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#singular: row.field(0usize + offset)?.into_string()?,
            r#adjective: row.field(1usize + offset)?.into_i8()?,
            r#plural: row.field(2usize + offset)?.into_string()?,
            r#possessive_pronoun: row.field(3usize + offset)?.into_i8()?,
            r#starts_with_vowel: row.field(4usize + offset)?.into_i8()?,
            r#unknown5: row.field(5usize + offset)?.into_i8()?,
            r#pronoun: row.field(6usize + offset)?.into_i8()?,
            r#article: row.field(7usize + offset)?.into_i8()?,
            r#place_name: row.field(8usize + offset)?.into_u16()?,
            r#housing_size: row.field(9usize + offset)?.into_u8()?,
            r#exterior_roof: row.field(10usize + offset)?.into_i32()?,
            r#exterior_wall: row.field(11usize + offset)?.into_i32()?,
            r#exterior_window: row.field(12usize + offset)?.into_i32()?,
            r#exterior_door: row.field(13usize + offset)?.into_i32()?,
            r#interior_wall: row.field(14usize + offset)?.into_i32()?,
            r#interior_flooring: row.field(15usize + offset)?.into_i32()?,
            r#interior_lighting: row.field(16usize + offset)?.into_i32()?,
            r#other_floor_wall: row.field(17usize + offset)?.into_i32()?,
            r#other_floor_flooring: row.field(18usize + offset)?.into_i32()?,
            r#other_floor_lighting: row.field(19usize + offset)?.into_i32()?,
            r#basement_wall: row.field(20usize + offset)?.into_i32()?,
            r#basement_flooring: row.field(21usize + offset)?.into_i32()?,
            r#basement_lighting: row.field(22usize + offset)?.into_i32()?,
            r#mansion_lighting: row.field(23usize + offset)?.into_i32()?,
        })
    }
}
