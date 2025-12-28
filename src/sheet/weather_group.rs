use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for WeatherGroup {
    fn name() -> String {
        "WeatherGroup".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WeatherGroup::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WeatherGroup {
    pub r#unknown0: i32,
    pub r#weather_rate: i32,
}
impl WeatherGroup {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i32()?,
            r#weather_rate: row.field(1usize + offset)?.into_i32()?,
        })
    }
}
