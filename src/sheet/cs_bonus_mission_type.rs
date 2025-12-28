use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for CSBonusMissionType {
    fn name() -> String {
        "CSBonusMissionType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CSBonusMissionType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CSBonusMissionType {
    pub r#unknown0: u8,
}
impl CSBonusMissionType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
