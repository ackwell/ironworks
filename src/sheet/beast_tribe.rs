use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for BeastTribe {
    fn name() -> String {
        "BeastTribe".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BeastTribe::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BeastTribe {
    pub r#unknown0: bool,
    pub r#min_level: u8,
    pub r#beast_rank_bonus: u8,
    pub r#icon_reputation: u32,
    pub r#icon: u32,
    pub r#max_rank: u8,
    pub r#expansion: u8,
    pub r#unknown7: u32,
    pub r#currency_item: u32,
    pub r#display_order: u32,
    pub r#name: u8,
    pub r#adjective: SeString,
    pub r#plural: i8,
    pub r#possessive_pronoun: SeString,
    pub r#starts_with_vowel: i8,
    pub r#pronoun: i8,
    pub r#article: i8,
    pub r#def: i8,
    pub r#name_relation: i8,
    pub r#unknown19: SeString,
}
impl BeastTribe {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#min_level: row.field(1usize + offset)?.into_u8()?,
            r#beast_rank_bonus: row.field(2usize + offset)?.into_u8()?,
            r#icon_reputation: row.field(3usize + offset)?.into_u32()?,
            r#icon: row.field(4usize + offset)?.into_u32()?,
            r#max_rank: row.field(5usize + offset)?.into_u8()?,
            r#expansion: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
            r#currency_item: row.field(8usize + offset)?.into_u32()?,
            r#display_order: row.field(9usize + offset)?.into_u32()?,
            r#name: row.field(10usize + offset)?.into_u8()?,
            r#adjective: row.field(11usize + offset)?.into_string()?,
            r#plural: row.field(12usize + offset)?.into_i8()?,
            r#possessive_pronoun: row.field(13usize + offset)?.into_string()?,
            r#starts_with_vowel: row.field(14usize + offset)?.into_i8()?,
            r#pronoun: row.field(15usize + offset)?.into_i8()?,
            r#article: row.field(16usize + offset)?.into_i8()?,
            r#def: row.field(17usize + offset)?.into_i8()?,
            r#name_relation: row.field(18usize + offset)?.into_i8()?,
            r#unknown19: row.field(19usize + offset)?.into_string()?,
        })
    }
}
