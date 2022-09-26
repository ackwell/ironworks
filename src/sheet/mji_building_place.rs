use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for MJIBuildingPlace {
    fn name() -> String {
        "MJIBuildingPlace".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIBuildingPlace::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIBuildingPlace {
    pub r#unknown0: u32,
    pub r#name: u32,
    pub r#sgb: u32,
}
impl MJIBuildingPlace {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#name: row.field(1usize + offset)?.into_u32()?,
            r#sgb: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
