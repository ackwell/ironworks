use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ItemRepairPrice {
    fn name() -> String {
        "ItemRepairPrice".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemRepairPrice::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemRepairPrice {}
impl ItemRepairPrice {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
