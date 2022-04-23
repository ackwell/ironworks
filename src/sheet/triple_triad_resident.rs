use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for TripleTriadResident {
    fn name() -> String {
        "TripleTriadResident".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TripleTriadResident::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TripleTriadResident {
    pub r#order: u16,
}
impl TripleTriadResident {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#order: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
