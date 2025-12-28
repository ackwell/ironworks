use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
pub struct FittingShopCategoryItem {
    pub r#unknown0: i32,
    pub r#unknown1: u8,
    pub r#unknown2: u8,
}
impl FittingShopCategoryItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
