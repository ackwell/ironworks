use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
pub struct MJIGatheringTool {
    pub r#item: u8,
}
impl MJIGatheringTool {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
