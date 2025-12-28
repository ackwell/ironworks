use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ContentExAction {
    fn name() -> String {
        "ContentExAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentExAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentExAction {
    pub r#name: u32,
    pub r#unknown1: u32,
    pub r#charges: u8,
    pub r#unknown3: u8,
}
impl ContentExAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
            r#charges: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
