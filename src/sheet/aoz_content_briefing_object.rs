use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for AOZContentBriefingObject {
    fn name() -> String {
        "AOZContentBriefingObject".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AOZContentBriefingObject::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AOZContentBriefingObject {
    pub r#icon: u32,
    pub r#unknown1: u16,
}
impl AOZContentBriefingObject {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
