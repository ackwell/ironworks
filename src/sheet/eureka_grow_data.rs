use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for EurekaGrowData {
    fn name() -> String {
        "EurekaGrowData".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EurekaGrowData::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EurekaGrowData {
    pub r#base_resistance: u16,
}
impl EurekaGrowData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#base_resistance: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
