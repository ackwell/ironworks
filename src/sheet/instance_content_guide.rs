use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
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
}
impl InstanceContentGuide {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#instance: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
