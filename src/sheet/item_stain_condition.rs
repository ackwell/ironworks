use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for ItemStainCondition {
    fn name() -> String {
        "ItemStainCondition".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemStainCondition::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemStainCondition {}
impl ItemStainCondition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
