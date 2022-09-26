use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
impl MetadataAdapter for Completion {
    fn name() -> String {
        "Completion".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Completion::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Completion {
    pub r#group: u16,
    pub r#key: u16,
    pub r#lookup_table: SeString,
    pub r#text: SeString,
    pub r#group_title: SeString,
}
impl Completion {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#group: row.field(0usize + offset)?.into_u16()?,
            r#key: row.field(1usize + offset)?.into_u16()?,
            r#lookup_table: row.field(2usize + offset)?.into_string()?,
            r#text: row.field(3usize + offset)?.into_string()?,
            r#group_title: row.field(4usize + offset)?.into_string()?,
        })
    }
}
