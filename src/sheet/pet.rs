use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Pet {
    fn name() -> String {
        "Pet".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Pet::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Pet {
    pub r#name: SeString,
}
impl Pet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
