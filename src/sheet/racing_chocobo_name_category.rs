use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
impl MetadataAdapter for RacingChocoboNameCategory {
    fn name() -> String {
        "RacingChocoboNameCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RacingChocoboNameCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RacingChocoboNameCategory {
    pub r#sort_key: u8,
    pub r#name: SeString,
}
impl RacingChocoboNameCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#sort_key: row.field(0usize + offset)?.into_u8()?,
            r#name: row.field(1usize + offset)?.into_string()?,
        })
    }
}
