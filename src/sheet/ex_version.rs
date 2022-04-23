use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for ExVersion {
    fn name() -> String {
        "ExVersion".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ExVersion::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ExVersion {
    pub r#name: SeString,
    pub r#accept_jingle: u16,
    pub r#complete_jingle: u16,
}
impl ExVersion {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#accept_jingle: row.field(1usize + offset)?.into_u16()?,
            r#complete_jingle: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
