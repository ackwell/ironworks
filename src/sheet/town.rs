use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for Town {
    fn name() -> String {
        "Town".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Town::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Town {
    pub r#name: SeString,
    pub r#icon: i32,
}
impl Town {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
        })
    }
}
