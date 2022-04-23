use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
impl MetadataAdapter for RacingChocoboName {
    fn name() -> String {
        "RacingChocoboName".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RacingChocoboName::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RacingChocoboName {
    pub r#name: SeString,
}
impl RacingChocoboName {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
