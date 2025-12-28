use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
    pub r#unknown1: i8,
}
impl ScenarioType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_i8()?,
        })
    }
}
