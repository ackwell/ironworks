use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for AOZBoss {
    fn name() -> String {
        "AOZBoss".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AOZBoss::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AOZBoss {
    pub r#boss: u16,
    pub r#position: u16,
}
impl AOZBoss {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#boss: row.field(0usize + offset)?.into_u16()?,
            r#position: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
