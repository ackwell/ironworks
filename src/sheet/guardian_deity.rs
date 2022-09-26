use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for GuardianDeity {
    fn name() -> String {
        "GuardianDeity".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GuardianDeity::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GuardianDeity {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#icon: u16,
}
impl GuardianDeity {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#icon: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
