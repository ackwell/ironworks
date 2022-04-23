use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
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
}
impl ChocoboRaceStatus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#status: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
