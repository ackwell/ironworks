use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for DeepDungeonMagicStone {
    fn name() -> String {
        "DeepDungeonMagicStone".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DeepDungeonMagicStone::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DeepDungeonMagicStone {
    pub r#icon: u32,
    pub r#singular: SeString,
    pub r#adjective: i8,
    pub r#plural: SeString,
    pub r#possessive_pronoun: i8,
    pub r#starts_with_vowel: i8,
    pub r#unknown6: i8,
    pub r#pronoun: i8,
    pub r#article: i8,
    pub r#name: SeString,
    pub r#tooltip: SeString,
}
impl DeepDungeonMagicStone {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_u32()?,
            r#singular: row.field(1usize + offset)?.into_string()?,
            r#adjective: row.field(2usize + offset)?.into_i8()?,
            r#plural: row.field(3usize + offset)?.into_string()?,
            r#possessive_pronoun: row.field(4usize + offset)?.into_i8()?,
            r#starts_with_vowel: row.field(5usize + offset)?.into_i8()?,
            r#unknown6: row.field(6usize + offset)?.into_i8()?,
            r#pronoun: row.field(7usize + offset)?.into_i8()?,
            r#article: row.field(8usize + offset)?.into_i8()?,
            r#name: row.field(9usize + offset)?.into_string()?,
            r#tooltip: row.field(10usize + offset)?.into_string()?,
        })
    }
}
