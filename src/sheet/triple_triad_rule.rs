use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for TripleTriadRule {
    fn name() -> String {
        "TripleTriadRule".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TripleTriadRule::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TripleTriadRule {
    pub r#name: SeString,
    pub r#description: SeString,
}
impl TripleTriadRule {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
        })
    }
}
