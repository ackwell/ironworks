use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
impl MetadataAdapter for CompanyLeveRule {
    fn name() -> String {
        "CompanyLeveRule".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompanyLeveRule::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompanyLeveRule {
    pub r#type: SeString,
    pub r#objective: u16,
    pub r#help: u16,
}
impl CompanyLeveRule {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_string()?,
            r#objective: row.field(1usize + offset)?.into_u16()?,
            r#help: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
