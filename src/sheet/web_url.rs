use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for WebURL {
    fn name() -> String {
        "WebURL".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WebURL::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WebURL {
    pub r#url: SeString,
}
impl WebURL {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#url: row.field(0usize + offset)?.into_string()?,
        })
    }
}
