use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Omikuji {
    fn name() -> String {
        "Omikuji".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Omikuji::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Omikuji {}
impl Omikuji {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
