use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for InstanceContentBuff {
    fn name() -> String {
        "InstanceContentBuff".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(InstanceContentBuff::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct InstanceContentBuff {
    pub r#echo_start: u16,
    pub r#echo_death: u16,
}
impl InstanceContentBuff {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#echo_start: row.field(0usize + offset)?.into_u16()?,
            r#echo_death: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
