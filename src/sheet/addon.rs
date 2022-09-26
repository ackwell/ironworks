use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
impl MetadataAdapter for Addon {
    fn name() -> String {
        "Addon".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Addon::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Addon {
    pub r#text: SeString,
}
impl Addon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
        })
    }
}
