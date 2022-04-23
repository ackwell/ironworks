use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::sestring::SeString;
impl MetadataAdapter for MainCommand {
    fn name() -> String {
        "MainCommand".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MainCommand::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MainCommand {
    pub r#icon: i32,
    pub r#category: u8,
    pub r#main_command_category: u8,
    pub r#sort_id: i8,
    pub r#name: u8,
    pub r#description: SeString,
}
impl MainCommand {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_i32()?,
            r#category: row.field(1usize + offset)?.into_u8()?,
            r#main_command_category: row.field(2usize + offset)?.into_u8()?,
            r#sort_id: row.field(3usize + offset)?.into_i8()?,
            r#name: row.field(4usize + offset)?.into_u8()?,
            r#description: row.field(5usize + offset)?.into_string()?,
        })
    }
}
