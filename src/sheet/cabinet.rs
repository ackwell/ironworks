use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#item: u32,
    pub r#order: u16,
    pub r#category: u8,
    pub r#sort_key: u8,
}
impl Cabinet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u32()?,
            r#order: row.field(1usize + offset)?.into_u16()?,
            r#category: row.field(2usize + offset)?.into_u8()?,
            r#sort_key: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
