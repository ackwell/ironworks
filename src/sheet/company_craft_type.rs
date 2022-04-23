use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
impl MetadataAdapter for CompanyCraftType {
    fn name() -> String {
        "CompanyCraftType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompanyCraftType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompanyCraftType {
    pub r#name: SeString,
}
impl CompanyCraftType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
