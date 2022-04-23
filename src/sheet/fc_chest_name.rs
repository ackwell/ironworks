use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for FCChestName {
    fn name() -> String {
        "FCChestName".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FCChestName::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FCChestName {
    pub r#name: SeString,
}
impl FCChestName {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
