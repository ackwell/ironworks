use ironworks::excel::Row;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for InstanceContentTextData {
    fn name() -> String {
        "InstanceContentTextData".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(InstanceContentTextData::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct InstanceContentTextData {
    pub r#text: SeString,
}
impl InstanceContentTextData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
        })
    }
}
