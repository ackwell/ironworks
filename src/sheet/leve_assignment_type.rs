use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
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
    pub r#is_faction: bool,
    pub r#icon: i32,
    pub r#name: SeString,
}
impl LeveAssignmentType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#is_faction: row.field(0usize + offset)?.into_bool()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#name: row.field(2usize + offset)?.into_string()?,
        })
    }
}
