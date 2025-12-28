use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for DescriptionSection {
    fn name() -> String {
        "DescriptionSection".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DescriptionSection::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DescriptionSection {
    pub r#string: u16,
    pub r#page: u16,
}
impl DescriptionSection {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#string: row.field(0usize + offset)?.into_u16()?,
            r#page: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
