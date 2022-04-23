use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for PartyContentTextData {
    fn name() -> String {
        "PartyContentTextData".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PartyContentTextData::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PartyContentTextData {
    pub r#data: SeString,
}
impl PartyContentTextData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#data: row.field(0usize + offset)?.into_string()?,
        })
    }
}
