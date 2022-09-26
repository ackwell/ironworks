use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for SharlayanCraftWorksSupply {
    fn name() -> String {
        "SharlayanCraftWorksSupply".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SharlayanCraftWorksSupply::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SharlayanCraftWorksSupply {}
impl SharlayanCraftWorksSupply {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
