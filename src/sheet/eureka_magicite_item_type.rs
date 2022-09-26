use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for EurekaMagiciteItemType {
    fn name() -> String {
        "EurekaMagiciteItemType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EurekaMagiciteItemType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EurekaMagiciteItemType {
    pub r#type: SeString,
}
impl EurekaMagiciteItemType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_string()?,
        })
    }
}
