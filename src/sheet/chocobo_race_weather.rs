use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for ChocoboRaceWeather {
    fn name() -> String {
        "ChocoboRaceWeather".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ChocoboRaceWeather::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ChocoboRaceWeather {
    pub r#weather_type1: i32,
    pub r#weather_type2: i32,
}
impl ChocoboRaceWeather {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#weather_type1: row.field(0usize + offset)?.into_i32()?,
            r#weather_type2: row.field(1usize + offset)?.into_i32()?,
        })
    }
}
