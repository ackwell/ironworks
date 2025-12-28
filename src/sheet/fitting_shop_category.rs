use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
pub struct FittingShopCategory {
    pub r#unknown0: u8,
    pub r#unknown1: SeString,
}
impl FittingShopCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_string()?,
        })
    }
}
