use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ArchiveItem {
    fn name() -> String {
        "ArchiveItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ArchiveItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ArchiveItem {
    pub r#unknown0: i32,
    pub r#unknown1: u16,
    pub r#unknown2: bool,
}
impl ArchiveItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
        })
    }
}
