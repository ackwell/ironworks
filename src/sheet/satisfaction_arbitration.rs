use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for SatisfactionArbitration {
    fn name() -> String {
        "SatisfactionArbitration".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SatisfactionArbitration::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SatisfactionArbitration {
    pub r#satisfaction_level: u8,
    pub r#satisfaction_npc: u8,
    pub r#quest: u32,
}
impl SatisfactionArbitration {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#satisfaction_level: row.field(0usize + offset)?.into_u8()?,
            r#satisfaction_npc: row.field(1usize + offset)?.into_u8()?,
            r#quest: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
