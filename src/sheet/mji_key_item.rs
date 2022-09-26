use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MJIKeyItem {
    fn name() -> String {
        "MJIKeyItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIKeyItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIKeyItem {
    pub r#item: i32,
}
impl MJIKeyItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
