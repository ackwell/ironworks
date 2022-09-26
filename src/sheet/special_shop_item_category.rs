use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
impl MetadataAdapter for SpecialShopItemCategory {
    fn name() -> String {
        "SpecialShopItemCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SpecialShopItemCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SpecialShopItemCategory {
    pub r#name: SeString,
}
impl SpecialShopItemCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
