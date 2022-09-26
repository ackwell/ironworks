use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for Title {
    fn name() -> String {
        "Title".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Title::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Title {
    pub r#masculine: SeString,
    pub r#feminine: SeString,
    pub r#is_prefix: bool,
    pub r#order: u16,
}
impl Title {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#masculine: row.field(0usize + offset)?.into_string()?,
            r#feminine: row.field(1usize + offset)?.into_string()?,
            r#is_prefix: row.field(2usize + offset)?.into_bool()?,
            r#order: row.field(3usize + offset)?.into_u16()?,
        })
    }
}
