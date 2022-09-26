use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for ChocoboRaceTerritory {
    fn name() -> String {
        "ChocoboRaceTerritory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ChocoboRaceTerritory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ChocoboRaceTerritory {
    pub r#name: u16,
    pub r#icon: i32,
}
impl ChocoboRaceTerritory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_u16()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
        })
    }
}
