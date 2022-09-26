use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use std::vec::Vec;
use ironworks::sestring::SeString;
use crate::utility::read_array;
use ironworks::excel::Row;
impl MetadataAdapter for Aetheryte {
    fn name() -> String {
        "Aetheryte".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Aetheryte::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Aetheryte {
    pub r#singular: SeString,
    pub r#adjective: i8,
    pub r#plural: SeString,
    pub r#possessive_pronoun: i8,
    pub r#starts_with_vowel: i8,
    pub r#unknown5: i8,
    pub r#pronoun: i8,
    pub r#article: i8,
    pub r#place_name: u16,
    pub r#aethernet_name: u16,
    pub r#territory: u16,
    pub r#level: Vec<u32>,
    pub r#is_aetheryte: bool,
    pub r#unknown16: SeString,
    pub r#aethernet_group: u8,
    pub r#invisible: bool,
    pub r#required_quest: u32,
    pub r#map: u16,
    pub r#aetherstream_x: i16,
    pub r#aetherstream_y: i16,
}
impl Aetheryte {
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
            r#aethernet_name: row.field(9usize + offset)?.into_u16()?,
            r#territory: row.field(10usize + offset)?.into_u16()?,
            r#level: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(11usize + offset)?.into_u32()?) },
            )?,
            r#is_aetheryte: row.field(15usize + offset)?.into_bool()?,
            r#unknown16: row.field(16usize + offset)?.into_string()?,
            r#aethernet_group: row.field(17usize + offset)?.into_u8()?,
            r#invisible: row.field(18usize + offset)?.into_bool()?,
            r#required_quest: row.field(19usize + offset)?.into_u32()?,
            r#map: row.field(20usize + offset)?.into_u16()?,
            r#aetherstream_x: row.field(21usize + offset)?.into_i16()?,
            r#aetherstream_y: row.field(22usize + offset)?.into_i16()?,
        })
    }
}
