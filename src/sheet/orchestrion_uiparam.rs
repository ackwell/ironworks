use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for OrchestrionUiparam {
    fn name() -> String {
        "OrchestrionUiparam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(OrchestrionUiparam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct OrchestrionUiparam {
    pub r#orchestrion_category: u8,
    pub r#order: u16,
}
impl OrchestrionUiparam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#orchestrion_category: row.field(0usize + offset)?.into_u8()?,
            r#order: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
