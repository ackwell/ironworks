use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for GuidePageString {
    fn name() -> String {
        "GuidePageString".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GuidePageString::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GuidePageString {
    pub r#string: SeString,
}
impl GuidePageString {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#string: row.field(0usize + offset)?.into_string()?,
        })
    }
}
