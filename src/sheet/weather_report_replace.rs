use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for WeatherReportReplace {
    fn name() -> String {
        "WeatherReportReplace".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WeatherReportReplace::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WeatherReportReplace {
    pub r#place_name_sub: u16,
    pub r#place_name_parent: u16,
}
impl WeatherReportReplace {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#place_name_sub: row.field(0usize + offset)?.into_u16()?,
            r#place_name_parent: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
