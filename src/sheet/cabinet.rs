use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Cabinet {
    fn name() -> String {
        "Cabinet".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Cabinet::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Cabinet {
    pub r#item: i32,
    pub r#order: u16,
    pub r#category: u8,
}
impl Cabinet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#order: row.field(1usize + offset)?.into_u16()?,
            r#category: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
