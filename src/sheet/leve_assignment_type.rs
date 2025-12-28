use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for LeveAssignmentType {
    fn name() -> String {
        "LeveAssignmentType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(LeveAssignmentType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct LeveAssignmentType {
    pub r#icon: i32,
    pub r#name: SeString,
}
impl LeveAssignmentType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_i32()?,
            r#name: row.field(1usize + offset)?.into_string()?,
        })
    }
}
