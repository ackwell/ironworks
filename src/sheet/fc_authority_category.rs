use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
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
