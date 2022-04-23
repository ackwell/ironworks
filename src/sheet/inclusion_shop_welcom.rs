use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for InclusionShopWelcom {
    fn name() -> String {
        "InclusionShopWelcom".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(InclusionShopWelcom::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct InclusionShopWelcom {}
impl InclusionShopWelcom {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
