use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for GCRankUldahFemaleText {
    fn name() -> String {
        "GCRankUldahFemaleText".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GCRankUldahFemaleText::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GCRankUldahFemaleText {
    pub r#singular: SeString,
    pub r#adjective: i8,
    pub r#plural: SeString,
    pub r#possessive_pronoun: i8,
    pub r#starts_with_vowel: i8,
    pub r#unknown5: i8,
    pub r#pronoun: i8,
    pub r#article: i8,
    pub r#name_rank: SeString,
    pub r#unknown9: SeString,
}
impl GCRankUldahFemaleText {
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
            r#name_rank: row.field(8usize + offset)?.into_string()?,
            r#unknown9: row.field(9usize + offset)?.into_string()?,
        })
    }
}
