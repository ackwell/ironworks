use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for RacingChocoboParam {
    fn name() -> String {
        "RacingChocoboParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RacingChocoboParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RacingChocoboParam {
    pub r#name: SeString,
}
impl RacingChocoboParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
