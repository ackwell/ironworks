use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for FittingShopCategory {
    fn name() -> String {
        "FittingShopCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FittingShopCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FittingShopCategory {}
impl FittingShopCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
