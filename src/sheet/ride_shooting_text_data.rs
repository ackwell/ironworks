use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for RideShootingTextData {
    fn name() -> String {
        "RideShootingTextData".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RideShootingTextData::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RideShootingTextData {
    pub r#string: SeString,
}
impl RideShootingTextData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#string: row.field(0usize + offset)?.into_string()?,
        })
    }
}
