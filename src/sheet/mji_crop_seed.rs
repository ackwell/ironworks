use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MJICropSeed {
    fn name() -> String {
        "MJICropSeed".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJICropSeed::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJICropSeed {
    pub r#item: u32,
    pub r#sgb: u16,
    pub r#name: u32,
}
impl MJICropSeed {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u32()?,
            r#sgb: row.field(1usize + offset)?.into_u16()?,
            r#name: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
