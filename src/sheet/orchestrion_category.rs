use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for OrchestrionCategory {
    fn name() -> String {
        "OrchestrionCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(OrchestrionCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct OrchestrionCategory {
    pub r#name: SeString,
    pub r#hide_order: u8,
    pub r#icon: u32,
    pub r#order: u8,
    pub r#unknown4: bool,
}
impl OrchestrionCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#hide_order: row.field(1usize + offset)?.into_u8()?,
            r#icon: row.field(2usize + offset)?.into_u32()?,
            r#order: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
        })
    }
}
