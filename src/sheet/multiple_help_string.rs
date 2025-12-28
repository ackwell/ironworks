use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for MultipleHelpString {
    fn name() -> String {
        "MultipleHelpString".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MultipleHelpString::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MultipleHelpString {
    pub r#unknown0: SeString,
    pub r#unknown1: SeString,
}
impl MultipleHelpString {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_string()?,
        })
    }
}
