use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for MYCTemporaryItemUICategory {
    fn name() -> String {
        "MYCTemporaryItemUICategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MYCTemporaryItemUICategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MYCTemporaryItemUICategory {
    pub r#name: SeString,
}
impl MYCTemporaryItemUICategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
