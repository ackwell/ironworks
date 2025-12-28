use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
    pub r#unknown3: u32,
    pub r#unknown4: u32,
}
impl ExVersion {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#accept_jingle: row.field(1usize + offset)?.into_u16()?,
            r#complete_jingle: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u32()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
        })
    }
}
