use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
impl MetadataAdapter for BuddyEquip {
    fn name() -> String {
        "BuddyEquip".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BuddyEquip::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BuddyEquip {
    pub r#singular: SeString,
    pub r#adjective: i8,
    pub r#plural: SeString,
    pub r#possessive_pronoun: i8,
    pub r#starts_with_vowel: i8,
    pub r#unknown5: i8,
    pub r#pronoun: i8,
    pub r#article: i8,
    pub r#name: SeString,
    pub r#model_top: i32,
    pub r#model_body: i32,
    pub r#model_legs: i32,
    pub r#grand_company: u8,
    pub r#icon_head: u16,
    pub r#icon_body: u16,
    pub r#icon_legs: u16,
    pub r#order: u8,
}
impl BuddyEquip {
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
            r#name: row.field(8usize + offset)?.into_string()?,
            r#model_top: row.field(9usize + offset)?.into_i32()?,
            r#model_body: row.field(10usize + offset)?.into_i32()?,
            r#model_legs: row.field(11usize + offset)?.into_i32()?,
            r#grand_company: row.field(12usize + offset)?.into_u8()?,
            r#icon_head: row.field(13usize + offset)?.into_u16()?,
            r#icon_body: row.field(14usize + offset)?.into_u16()?,
            r#icon_legs: row.field(15usize + offset)?.into_u16()?,
            r#order: row.field(16usize + offset)?.into_u8()?,
        })
    }
}
