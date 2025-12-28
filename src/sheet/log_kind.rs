use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
    pub r#format: bool,
    pub r#unknown2: SeString,
    pub r#unknown3: u8,
}
impl LogKind {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#format: row.field(1usize + offset)?.into_bool()?,
            r#unknown2: row.field(2usize + offset)?.into_string()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
