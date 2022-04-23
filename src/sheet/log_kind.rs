use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
impl MetadataAdapter for LogKind {
    fn name() -> String {
        "LogKind".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(LogKind::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct LogKind {
    pub r#unknown0: u8,
    pub r#format: SeString,
}
impl LogKind {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#format: row.field(1usize + offset)?.into_string()?,
        })
    }
}
