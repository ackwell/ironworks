use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for FittingShopItemSet {
    fn name() -> String {
        "FittingShopItemSet".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FittingShopItemSet::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FittingShopItemSet {}
impl FittingShopItemSet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
