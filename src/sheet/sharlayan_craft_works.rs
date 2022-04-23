use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for SharlayanCraftWorks {
    fn name() -> String {
        "SharlayanCraftWorks".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SharlayanCraftWorks::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SharlayanCraftWorks {}
impl SharlayanCraftWorks {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
