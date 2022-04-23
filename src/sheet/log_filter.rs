use std::result::Result;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for LogFilter {
    fn name() -> String {
        "LogFilter".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(LogFilter::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct LogFilter {
    pub r#log_kind: u8,
    pub r#caster: u16,
    pub r#target: u16,
    pub r#category: u8,
    pub r#display_order: u8,
    pub r#preset: u8,
    pub r#name: SeString,
    pub r#example: SeString,
}
impl LogFilter {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#log_kind: row.field(0usize + offset)?.into_u8()?,
            r#caster: row.field(1usize + offset)?.into_u16()?,
            r#target: row.field(2usize + offset)?.into_u16()?,
            r#category: row.field(3usize + offset)?.into_u8()?,
            r#display_order: row.field(4usize + offset)?.into_u8()?,
            r#preset: row.field(5usize + offset)?.into_u8()?,
            r#name: row.field(6usize + offset)?.into_string()?,
            r#example: row.field(7usize + offset)?.into_string()?,
        })
    }
}
