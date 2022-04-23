use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for RelicNoteCategory {
    fn name() -> String {
        "RelicNoteCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RelicNoteCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RelicNoteCategory {
    pub r#unknown0: i8,
    pub r#text: SeString,
}
impl RelicNoteCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i8()?,
            r#text: row.field(1usize + offset)?.into_string()?,
        })
    }
}
