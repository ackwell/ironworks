use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for Ornament {
    fn name() -> String {
        "Ornament".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Ornament::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Ornament {
    pub r#model: u16,
    pub r#unknown1: u8,
    pub r#unknown2: u8,
    pub r#unknown3: bool,
    pub r#order: u8,
    pub r#icon: u16,
    pub r#transient: i16,
    pub r#singular: u16,
    pub r#adjective: u16,
    pub r#plural: SeString,
    pub r#possessive_pronoun: i8,
    pub r#starts_with_vowel: SeString,
    pub r#unknown12: i8,
    pub r#pronoun: i8,
    pub r#article: i8,
}
impl Ornament {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#model: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#order: row.field(4usize + offset)?.into_u8()?,
            r#icon: row.field(5usize + offset)?.into_u16()?,
            r#transient: row.field(6usize + offset)?.into_i16()?,
            r#singular: row.field(7usize + offset)?.into_u16()?,
            r#adjective: row.field(8usize + offset)?.into_u16()?,
            r#plural: row.field(9usize + offset)?.into_string()?,
            r#possessive_pronoun: row.field(10usize + offset)?.into_i8()?,
            r#starts_with_vowel: row.field(11usize + offset)?.into_string()?,
            r#unknown12: row.field(12usize + offset)?.into_i8()?,
            r#pronoun: row.field(13usize + offset)?.into_i8()?,
            r#article: row.field(14usize + offset)?.into_i8()?,
        })
    }
}
