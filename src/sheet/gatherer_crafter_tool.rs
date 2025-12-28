use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for GathererCrafterTool {
    fn name() -> String {
        "GathererCrafterTool".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GathererCrafterTool::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GathererCrafterTool {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
}
impl GathererCrafterTool {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
