use ironworks::sestring::SeString;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for CompanionMove {
    fn name() -> String {
        "CompanionMove".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompanionMove::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompanionMove {
    pub r#name: SeString,
}
impl CompanionMove {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
