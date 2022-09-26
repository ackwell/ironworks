use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for JournalGenre {
    fn name() -> String {
        "JournalGenre".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(JournalGenre::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct JournalGenre {
    pub r#icon: i32,
    pub r#journal_category: u8,
    pub r#unknown2: bool,
    pub r#name: SeString,
}
impl JournalGenre {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_i32()?,
            r#journal_category: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#name: row.field(3usize + offset)?.into_string()?,
        })
    }
}
