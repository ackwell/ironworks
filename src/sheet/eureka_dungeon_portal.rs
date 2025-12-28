use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for EurekaDungeonPortal {
    fn name() -> String {
        "EurekaDungeonPortal".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EurekaDungeonPortal::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EurekaDungeonPortal {
    pub r#level_id: u32,
}
impl EurekaDungeonPortal {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#level_id: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
