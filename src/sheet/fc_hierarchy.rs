use std::result::Result;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for FCHierarchy {
    fn name() -> String {
        "FCHierarchy".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FCHierarchy::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FCHierarchy {
    pub r#name: SeString,
}
impl FCHierarchy {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
