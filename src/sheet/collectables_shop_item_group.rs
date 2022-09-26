use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
impl MetadataAdapter for CollectablesShopItemGroup {
    fn name() -> String {
        "CollectablesShopItemGroup".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CollectablesShopItemGroup::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CollectablesShopItemGroup {
    pub r#name: SeString,
}
impl CollectablesShopItemGroup {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
