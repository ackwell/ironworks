use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
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
    pub r#unknown3: u8,
    pub r#unknown4: u16,
    pub r#order: i16,
    pub r#icon: u16,
    pub r#transient: u16,
    pub r#singular: SeString,
    pub r#adjective: i8,
    pub r#plural: SeString,
    pub r#possessive_pronoun: i8,
    pub r#starts_with_vowel: i8,
    pub r#unknown13: i8,
    pub r#pronoun: i8,
    pub r#article: i8,
}
impl Ornament {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#model: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#order: row.field(5usize + offset)?.into_i16()?,
            r#icon: row.field(6usize + offset)?.into_u16()?,
            r#transient: row.field(7usize + offset)?.into_u16()?,
            r#singular: row.field(8usize + offset)?.into_string()?,
            r#adjective: row.field(9usize + offset)?.into_i8()?,
            r#plural: row.field(10usize + offset)?.into_string()?,
            r#possessive_pronoun: row.field(11usize + offset)?.into_i8()?,
            r#starts_with_vowel: row.field(12usize + offset)?.into_i8()?,
            r#unknown13: row.field(13usize + offset)?.into_i8()?,
            r#pronoun: row.field(14usize + offset)?.into_i8()?,
            r#article: row.field(15usize + offset)?.into_i8()?,
        })
    }
}
