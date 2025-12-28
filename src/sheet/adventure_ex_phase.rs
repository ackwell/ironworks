use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for AdventureExPhase {
    fn name() -> String {
        "AdventureExPhase".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AdventureExPhase::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AdventureExPhase {
    pub r#quest: u32,
    pub r#adventure_begin: u32,
    pub r#adventure_end: u32,
    pub r#expansion: u8,
    pub r#unknown4: u32,
}
impl AdventureExPhase {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_u32()?,
            r#adventure_begin: row.field(1usize + offset)?.into_u32()?,
            r#adventure_end: row.field(2usize + offset)?.into_u32()?,
            r#expansion: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
        })
    }
}
