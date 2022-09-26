use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MJIText {
    fn name() -> String {
        "MJIText".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIText::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIText {
    pub r#text: SeString,
}
impl MJIText {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
        })
    }
}
