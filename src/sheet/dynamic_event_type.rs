use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for DynamicEventType {
    fn name() -> String {
        "DynamicEventType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DynamicEventType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DynamicEventType {
    pub r#icon_objective0: u32,
    pub r#icon_objective1: u32,
}
impl DynamicEventType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon_objective0: row.field(0usize + offset)?.into_u32()?,
            r#icon_objective1: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
