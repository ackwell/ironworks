use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
impl MetadataAdapter for EventItem {
    fn name() -> String {
        "EventItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EventItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EventItem {
    pub r#singular: SeString,
    pub r#adjective: i8,
    pub r#plural: SeString,
    pub r#possessive_pronoun: i8,
    pub r#starts_with_vowel: i8,
    pub r#unknown5: i8,
    pub r#pronoun: i8,
    pub r#article: i8,
    pub r#unknown8: bool,
    pub r#name: SeString,
    pub r#icon: u16,
    pub r#action: u16,
    pub r#stack_size: u8,
    pub r#unknown13: u8,
    pub r#quest: u32,
    pub r#cast_time: u8,
    pub r#cast_timeline: u8,
    pub r#timeline: u8,
}
impl EventItem {
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
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
            r#name: row.field(9usize + offset)?.into_string()?,
            r#icon: row.field(10usize + offset)?.into_u16()?,
            r#action: row.field(11usize + offset)?.into_u16()?,
            r#stack_size: row.field(12usize + offset)?.into_u8()?,
            r#unknown13: row.field(13usize + offset)?.into_u8()?,
            r#quest: row.field(14usize + offset)?.into_u32()?,
            r#cast_time: row.field(15usize + offset)?.into_u8()?,
            r#cast_timeline: row.field(16usize + offset)?.into_u8()?,
            r#timeline: row.field(17usize + offset)?.into_u8()?,
        })
    }
}
