use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for OpenContentCandidateName {
    fn name() -> String {
        "OpenContentCandidateName".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(OpenContentCandidateName::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct OpenContentCandidateName {
    pub r#name: SeString,
}
impl OpenContentCandidateName {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
