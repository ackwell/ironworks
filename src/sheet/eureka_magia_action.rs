use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for EurekaMagiaAction {
    fn name() -> String {
        "EurekaMagiaAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EurekaMagiaAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EurekaMagiaAction {
    pub r#action: u32,
    pub r#max_uses: u8,
}
impl EurekaMagiaAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#action: row.field(0usize + offset)?.into_u32()?,
            r#max_uses: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
