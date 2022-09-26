use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MJIGatheringTool {
    fn name() -> String {
        "MJIGatheringTool".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIGatheringTool::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIGatheringTool {}
impl MJIGatheringTool {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
