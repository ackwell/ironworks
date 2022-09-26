use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Frontline04 {
    fn name() -> String {
        "Frontline04".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Frontline04::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Frontline04 {
    pub r#level1: i32,
    pub r#level2: i32,
    pub r#level3: i32,
}
impl Frontline04 {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#level1: row.field(0usize + offset)?.into_i32()?,
            r#level2: row.field(1usize + offset)?.into_i32()?,
            r#level3: row.field(2usize + offset)?.into_i32()?,
        })
    }
}
