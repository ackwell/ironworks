use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for InclusionShopWelcomText {
    fn name() -> String {
        "InclusionShopWelcomText".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(InclusionShopWelcomText::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct InclusionShopWelcomText {}
impl InclusionShopWelcomText {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
