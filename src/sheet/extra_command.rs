use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for ExtraCommand {
    fn name() -> String {
        "ExtraCommand".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ExtraCommand::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ExtraCommand {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#icon: i32,
    pub r#order: i8,
}
impl ExtraCommand {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#icon: row.field(2usize + offset)?.into_i32()?,
            r#order: row.field(3usize + offset)?.into_i8()?,
        })
    }
}
