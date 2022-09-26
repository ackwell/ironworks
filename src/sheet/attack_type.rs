use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for AttackType {
    fn name() -> String {
        "AttackType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AttackType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AttackType {
    pub r#name: SeString,
}
impl AttackType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
