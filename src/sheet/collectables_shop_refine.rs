use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for CollectablesShopRefine {
    fn name() -> String {
        "CollectablesShopRefine".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CollectablesShopRefine::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CollectablesShopRefine {
    pub r#low_collectability: u16,
    pub r#mid_collectability: u16,
    pub r#high_collectability: u16,
}
impl CollectablesShopRefine {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#low_collectability: row.field(0usize + offset)?.into_u16()?,
            r#mid_collectability: row.field(1usize + offset)?.into_u16()?,
            r#high_collectability: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
