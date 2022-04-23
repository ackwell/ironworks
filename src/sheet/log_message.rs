use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for LogMessage {
    fn name() -> String {
        "LogMessage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(LogMessage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct LogMessage {
    pub r#log_kind: u16,
    pub r#unknown1: u16,
    pub r#unknown2: u8,
    pub r#unknown3: bool,
    pub r#text: SeString,
}
impl LogMessage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#log_kind: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#text: row.field(4usize + offset)?.into_string()?,
        })
    }
}
