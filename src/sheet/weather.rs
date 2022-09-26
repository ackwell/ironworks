use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for Weather {
    fn name() -> String {
        "Weather".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Weather::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Weather {
    pub r#icon: i32,
    pub r#name: SeString,
    pub r#description: SeString,
}
impl Weather {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_i32()?,
            r#name: row.field(1usize + offset)?.into_string()?,
            r#description: row.field(2usize + offset)?.into_string()?,
        })
    }
}
