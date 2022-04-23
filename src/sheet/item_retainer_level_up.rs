use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for ItemRetainerLevelUp {
    fn name() -> String {
        "ItemRetainerLevelUp".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemRetainerLevelUp::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemRetainerLevelUp {}
impl ItemRetainerLevelUp {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
