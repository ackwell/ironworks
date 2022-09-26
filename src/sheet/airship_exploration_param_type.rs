use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for AirshipExplorationParamType {
    fn name() -> String {
        "AirshipExplorationParamType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AirshipExplorationParamType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AirshipExplorationParamType {
    pub r#name: SeString,
}
impl AirshipExplorationParamType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
