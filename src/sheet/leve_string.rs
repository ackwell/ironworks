use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for LeveString {
    fn name() -> String {
        "LeveString".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(LeveString::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct LeveString {
    pub r#objective: SeString,
}
impl LeveString {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#objective: row.field(0usize + offset)?.into_string()?,
        })
    }
}
