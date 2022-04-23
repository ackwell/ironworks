use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for CreditListText {
    fn name() -> String {
        "CreditListText".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CreditListText::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CreditListText {
    pub r#name: SeString,
}
impl CreditListText {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
