use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for InstanceContentCSBonus {
    fn name() -> String {
        "InstanceContentCSBonus".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(InstanceContentCSBonus::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct InstanceContentCSBonus {
    pub r#instance: u16,
    pub r#item: u32,
}
impl InstanceContentCSBonus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#instance: row.field(0usize + offset)?.into_u16()?,
            r#item: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
