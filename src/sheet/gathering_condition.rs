use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
impl MetadataAdapter for GatheringCondition {
    fn name() -> String {
        "GatheringCondition".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringCondition::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringCondition {
    pub r#text: SeString,
}
impl GatheringCondition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
        })
    }
}
