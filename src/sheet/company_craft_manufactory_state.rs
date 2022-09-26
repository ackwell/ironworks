use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
impl MetadataAdapter for CompanyCraftManufactoryState {
    fn name() -> String {
        "CompanyCraftManufactoryState".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompanyCraftManufactoryState::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompanyCraftManufactoryState {
    pub r#name: SeString,
}
impl CompanyCraftManufactoryState {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
