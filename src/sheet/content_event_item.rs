use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ContentEventItem {
    fn name() -> String {
        "ContentEventItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentEventItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentEventItem {
    pub r#unknown0: u32,
}
impl ContentEventItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
