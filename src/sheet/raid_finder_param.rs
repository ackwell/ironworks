use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for RaidFinderParam {
    fn name() -> String {
        "RaidFinderParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RaidFinderParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RaidFinderParam {}
impl RaidFinderParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
