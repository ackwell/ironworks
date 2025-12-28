use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for InclusionShopSeries {
    fn name() -> String {
        "InclusionShopSeries".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(InclusionShopSeries::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct InclusionShopSeries {
    pub r#special_shop: u32,
}
impl InclusionShopSeries {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#special_shop: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
