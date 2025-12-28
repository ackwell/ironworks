use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for JournalCategory {
    fn name() -> String {
        "JournalCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(JournalCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct JournalCategory {
    pub r#name: SeString,
    pub r#separate_type: u8,
    pub r#data_type: u8,
    pub r#journal_section: u8,
    pub r#unknown4: u8,
}
impl JournalCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#separate_type: row.field(1usize + offset)?.into_u8()?,
            r#data_type: row.field(2usize + offset)?.into_u8()?,
            r#journal_section: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
