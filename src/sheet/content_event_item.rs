use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for ContentEventItem {
    fn name() -> String {
        "ContentEventItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentEventItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentEventItem {}
impl ContentEventItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
