use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
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
pub struct ArchiveItem {}
impl ArchiveItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
