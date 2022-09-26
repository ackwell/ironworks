use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
impl MetadataAdapter for SnipeTalkName {
    fn name() -> String {
        "SnipeTalkName".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SnipeTalkName::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SnipeTalkName {
    pub r#name: SeString,
}
impl SnipeTalkName {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
