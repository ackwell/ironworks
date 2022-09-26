use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for AOZReport {
    fn name() -> String {
        "AOZReport".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AOZReport::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AOZReport {
    pub r#unknown0: u32,
    pub r#reward: u8,
    pub r#order: i8,
}
impl AOZReport {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#reward: row.field(1usize + offset)?.into_u8()?,
            r#order: row.field(2usize + offset)?.into_i8()?,
        })
    }
}
