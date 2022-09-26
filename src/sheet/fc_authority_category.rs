use std::result::Result;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for FCAuthorityCategory {
    fn name() -> String {
        "FCAuthorityCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FCAuthorityCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FCAuthorityCategory {
    pub r#name: SeString,
}
impl FCAuthorityCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
