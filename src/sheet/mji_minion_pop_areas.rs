use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for MJIMinionPopAreas {
    fn name() -> String {
        "MJIMinionPopAreas".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIMinionPopAreas::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIMinionPopAreas {}
impl MJIMinionPopAreas {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
