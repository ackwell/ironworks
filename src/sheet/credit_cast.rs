use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for CreditCast {
    fn name() -> String {
        "CreditCast".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CreditCast::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CreditCast {
    pub r#name: SeString,
}
impl CreditCast {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
