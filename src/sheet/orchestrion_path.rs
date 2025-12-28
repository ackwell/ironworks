use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for OrchestrionPath {
    fn name() -> String {
        "OrchestrionPath".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(OrchestrionPath::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct OrchestrionPath {
    pub r#file: SeString,
}
impl OrchestrionPath {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#file: row.field(0usize + offset)?.into_string()?,
        })
    }
}
