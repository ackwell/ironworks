use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for MultipleHelp {
    fn name() -> String {
        "MultipleHelp".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MultipleHelp::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MultipleHelp {
    pub r#unknown0: u16,
    pub r#unknown1: SeString,
    pub r#unknown2: SeString,
}
impl MultipleHelp {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_string()?,
            r#unknown2: row.field(2usize + offset)?.into_string()?,
        })
    }
}
