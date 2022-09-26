use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
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
