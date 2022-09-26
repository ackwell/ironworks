use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for TripleTriadCardObtain {
    fn name() -> String {
        "TripleTriadCardObtain".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TripleTriadCardObtain::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TripleTriadCardObtain {}
impl TripleTriadCardObtain {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
