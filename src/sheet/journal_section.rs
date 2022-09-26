use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for JournalSection {
    fn name() -> String {
        "JournalSection".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(JournalSection::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct JournalSection {
    pub r#name: SeString,
}
impl JournalSection {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
