use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for PvPTrait {
    fn name() -> String {
        "PvPTrait".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PvPTrait::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PvPTrait {
    pub r#trait1: u16,
    pub r#trait2: u16,
    pub r#trait3: u16,
}
impl PvPTrait {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#trait1: row.field(0usize + offset)?.into_u16()?,
            r#trait2: row.field(1usize + offset)?.into_u16()?,
            r#trait3: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
