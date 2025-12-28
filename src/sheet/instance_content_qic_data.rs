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
pub struct InstanceContentQICData {
    pub r#unknown0: u8,
    pub r#unknown1: bool,
}
impl InstanceContentQICData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
        })
    }
}
