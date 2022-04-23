use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for PublicContentTextData {
    fn name() -> String {
        "PublicContentTextData".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PublicContentTextData::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PublicContentTextData {
    pub r#text_data: SeString,
}
impl PublicContentTextData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text_data: row.field(0usize + offset)?.into_string()?,
        })
    }
}
