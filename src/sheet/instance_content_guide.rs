use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for InstanceContentGuide {
    fn name() -> String {
        "InstanceContentGuide".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(InstanceContentGuide::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct InstanceContentGuide {
    pub r#instance: u32,
    pub r#unknown1: u32,
}
impl InstanceContentGuide {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#instance: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
