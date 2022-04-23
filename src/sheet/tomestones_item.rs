use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for TomestonesItem {
    fn name() -> String {
        "TomestonesItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TomestonesItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TomestonesItem {
    pub r#item: i32,
    pub r#unknown1: i8,
    pub r#tomestones: i32,
}
impl TomestonesItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_i8()?,
            r#tomestones: row.field(2usize + offset)?.into_i32()?,
        })
    }
}
