use crate::error::PopulateError;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for TripleTriadCardType {
    fn name() -> String {
        "TripleTriadCardType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TripleTriadCardType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TripleTriadCardType {
    pub r#name: SeString,
}
impl TripleTriadCardType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
