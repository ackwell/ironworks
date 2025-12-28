use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for TextCommand {
    fn name() -> String {
        "TextCommand".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TextCommand::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TextCommand {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#unknown2: i8,
    pub r#unknown3: i8,
    pub r#unknown4: i8,
    pub r#command: SeString,
    pub r#short_command: SeString,
    pub r#description: SeString,
    pub r#alias: SeString,
    pub r#short_alias: SeString,
    pub r#param: u16,
    pub r#unknown11: u32,
}
impl TextCommand {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_i8()?,
            r#unknown3: row.field(3usize + offset)?.into_i8()?,
            r#unknown4: row.field(4usize + offset)?.into_i8()?,
            r#command: row.field(5usize + offset)?.into_string()?,
            r#short_command: row.field(6usize + offset)?.into_string()?,
            r#description: row.field(7usize + offset)?.into_string()?,
            r#alias: row.field(8usize + offset)?.into_string()?,
            r#short_alias: row.field(9usize + offset)?.into_string()?,
            r#param: row.field(10usize + offset)?.into_u16()?,
            r#unknown11: row.field(11usize + offset)?.into_u32()?,
        })
    }
}
