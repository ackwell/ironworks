use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for FittingShopCategoryItem {
    fn name() -> String {
        "FittingShopCategoryItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FittingShopCategoryItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FittingShopCategoryItem {}
impl FittingShopCategoryItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
