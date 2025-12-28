use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ChocoboRaceStatus {
    fn name() -> String {
        "ChocoboRaceStatus".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ChocoboRaceStatus::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ChocoboRaceStatus {
    pub r#status: i32,
    pub r#unknown1: u16,
}
impl ChocoboRaceStatus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#status: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
