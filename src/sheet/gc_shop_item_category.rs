use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
impl MetadataAdapter for GCShopItemCategory {
    fn name() -> String {
        "GCShopItemCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GCShopItemCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GCShopItemCategory {
    pub r#name: SeString,
}
impl GCShopItemCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
