use crate::error::PopulateError;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for DescriptionString {
    fn name() -> String {
        "DescriptionString".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DescriptionString::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DescriptionString {
    pub r#text: SeString,
}
impl DescriptionString {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
        })
    }
}
