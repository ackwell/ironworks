use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for WeatherRate {
    fn name() -> String {
        "WeatherRate".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WeatherRate::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WeatherRate_ate {
    pub r#weather: i32,
    pub r#rate: u8,
}
impl WeatherRate_ate {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#weather: row.field(0usize + offset)?.into_i32()?,
            r#rate: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct WeatherRate {
    pub r#ate: Vec<WeatherRate_ate>,
}
impl WeatherRate {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#ate: read_array(
                offset,
                8usize,
                2usize,
                |offset| { Result::Ok(WeatherRate_ate::populate(row, offset)?) },
            )?,
        })
    }
}
