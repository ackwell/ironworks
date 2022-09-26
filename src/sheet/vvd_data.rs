use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for VVDData {
    fn name() -> String {
        "VVDData".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(VVDData::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct VVDData {}
impl VVDData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
