use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ActionCategory {
    fn name() -> String {
        "ActionCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActionCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActionCategory {
    pub r#name: SeString,
}
impl ActionCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
