use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for HWDLevelChangeDeception {
    fn name() -> String {
        "HWDLevelChangeDeception".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDLevelChangeDeception::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDLevelChangeDeception {
    pub r#image: i32,
}
impl HWDLevelChangeDeception {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#image: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
