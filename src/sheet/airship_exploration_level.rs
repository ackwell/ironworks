use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for AirshipExplorationLevel {
    fn name() -> String {
        "AirshipExplorationLevel".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AirshipExplorationLevel::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AirshipExplorationLevel {
    pub r#capacity: u16,
    pub r#exp_to_next: u32,
}
impl AirshipExplorationLevel {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#capacity: row.field(0usize + offset)?.into_u16()?,
            r#exp_to_next: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
