use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for ContentRandomSelect {
    fn name() -> String {
        "ContentRandomSelect".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentRandomSelect::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentRandomSelect {
    pub r#name: u16,
}
impl ContentRandomSelect {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
