use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for LegacyQuest {
    fn name() -> String {
        "LegacyQuest".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(LegacyQuest::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct LegacyQuest {
    pub r#legacy_quest_id: u16,
    pub r#text: SeString,
    pub r#string: SeString,
    pub r#sort_key: u16,
    pub r#genre: u32,
}
impl LegacyQuest {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#legacy_quest_id: row.field(0usize + offset)?.into_u16()?,
            r#text: row.field(1usize + offset)?.into_string()?,
            r#string: row.field(2usize + offset)?.into_string()?,
            r#sort_key: row.field(3usize + offset)?.into_u16()?,
            r#genre: row.field(4usize + offset)?.into_u32()?,
        })
    }
}
