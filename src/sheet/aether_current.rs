use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for AetherCurrent {
    fn name() -> String {
        "AetherCurrent".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AetherCurrent::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AetherCurrent {
    pub r#quest: u32,
}
impl AetherCurrent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
