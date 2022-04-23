use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for DynamicEventEnemyType {
    fn name() -> String {
        "DynamicEventEnemyType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DynamicEventEnemyType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DynamicEventEnemyType {
    pub r#name: SeString,
}
impl DynamicEventEnemyType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
