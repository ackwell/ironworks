use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for EmjDani {
    fn name() -> String {
        "EmjDani".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EmjDani::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EmjDani {
    pub r#icon: u32,
}
impl EmjDani {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
