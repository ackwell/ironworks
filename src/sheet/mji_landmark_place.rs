use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for MJILandmarkPlace {
    fn name() -> String {
        "MJILandmarkPlace".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJILandmarkPlace::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJILandmarkPlace {
    pub r#unknown0: u32,
    pub r#name: u32,
    pub r#sgb: u32,
}
impl MJILandmarkPlace {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#name: row.field(1usize + offset)?.into_u32()?,
            r#sgb: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
