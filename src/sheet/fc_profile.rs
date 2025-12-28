use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for FCProfile {
    fn name() -> String {
        "FCProfile".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FCProfile::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FCProfile {
    pub r#priority: u8,
    pub r#name: SeString,
}
impl FCProfile {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#priority: row.field(0usize + offset)?.into_u8()?,
            r#name: row.field(1usize + offset)?.into_string()?,
        })
    }
}
