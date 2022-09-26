use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
impl MetadataAdapter for ScenarioType {
    fn name() -> String {
        "ScenarioType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ScenarioType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ScenarioType {
    pub r#type: SeString,
}
impl ScenarioType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_string()?,
        })
    }
}
