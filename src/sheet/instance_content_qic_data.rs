use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for InstanceContentQICData {
    fn name() -> String {
        "InstanceContentQICData".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(InstanceContentQICData::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct InstanceContentQICData {}
impl InstanceContentQICData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
