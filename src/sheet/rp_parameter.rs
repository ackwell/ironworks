use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for RPParameter {
    fn name() -> String {
        "RPParameter".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RPParameter::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RPParameter {
    pub r#b_npc_name: u16,
    pub r#class_job: u8,
    pub r#unknown2: u8,
}
impl RPParameter {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#b_npc_name: row.field(0usize + offset)?.into_u16()?,
            r#class_job: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
