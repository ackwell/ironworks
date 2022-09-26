use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for VVDRouteData {
    fn name() -> String {
        "VVDRouteData".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(VVDRouteData::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct VVDRouteData {}
impl VVDRouteData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
