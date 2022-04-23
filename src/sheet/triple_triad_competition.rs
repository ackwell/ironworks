use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
impl MetadataAdapter for TripleTriadCompetition {
    fn name() -> String {
        "TripleTriadCompetition".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TripleTriadCompetition::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TripleTriadCompetition {
    pub r#name: SeString,
}
impl TripleTriadCompetition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
