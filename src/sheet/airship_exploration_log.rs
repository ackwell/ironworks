use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for AirshipExplorationLog {
    fn name() -> String {
        "AirshipExplorationLog".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AirshipExplorationLog::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AirshipExplorationLog {
    pub r#text: SeString,
}
impl AirshipExplorationLog {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
        })
    }
}
