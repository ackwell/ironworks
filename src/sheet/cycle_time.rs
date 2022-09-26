use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for CycleTime {
    fn name() -> String {
        "CycleTime".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CycleTime::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CycleTime {
    pub r#first_cycle: u32,
    pub r#cycle: u32,
}
impl CycleTime {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#first_cycle: row.field(0usize + offset)?.into_u32()?,
            r#cycle: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
