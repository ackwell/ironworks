use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for IconLanguage {
    fn name() -> String {
        "IconLanguage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(IconLanguage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct IconLanguage {}
impl IconLanguage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
