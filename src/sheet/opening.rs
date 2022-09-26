use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
impl MetadataAdapter for Opening {
    fn name() -> String {
        "Opening".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Opening::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Opening {
    pub r#name: SeString,
    pub r#quest: u32,
}
impl Opening {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#quest: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
