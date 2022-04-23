use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for ChocoboRace {
    fn name() -> String {
        "ChocoboRace".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ChocoboRace::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ChocoboRace {
    pub r#chocobo_race_rank: u8,
    pub r#chocobo_race_territory: u8,
}
impl ChocoboRace {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#chocobo_race_rank: row.field(0usize + offset)?.into_u8()?,
            r#chocobo_race_territory: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
