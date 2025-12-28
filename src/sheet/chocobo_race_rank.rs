use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ChocoboRaceRank {
    fn name() -> String {
        "ChocoboRaceRank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ChocoboRaceRank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ChocoboRaceRank {
    pub r#rating_min: u16,
    pub r#rating_max: u16,
    pub r#name: u16,
    pub r#fee: u16,
    pub r#icon: i32,
}
impl ChocoboRaceRank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#rating_min: row.field(0usize + offset)?.into_u16()?,
            r#rating_max: row.field(1usize + offset)?.into_u16()?,
            r#name: row.field(2usize + offset)?.into_u16()?,
            r#fee: row.field(3usize + offset)?.into_u16()?,
            r#icon: row.field(4usize + offset)?.into_i32()?,
        })
    }
}
