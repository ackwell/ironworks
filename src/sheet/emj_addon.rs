use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for EmjAddon {
    fn name() -> String {
        "EmjAddon".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EmjAddon::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EmjAddon {
    pub r#text: SeString,
}
impl EmjAddon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
        })
    }
}
