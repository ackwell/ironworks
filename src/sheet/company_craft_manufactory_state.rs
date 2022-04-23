use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
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
