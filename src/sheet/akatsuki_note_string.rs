use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for AkatsukiNoteString {
    fn name() -> String {
        "AkatsukiNoteString".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AkatsukiNoteString::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AkatsukiNoteString {}
impl AkatsukiNoteString {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
