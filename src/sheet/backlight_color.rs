use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for BacklightColor {
    fn name() -> String {
        "BacklightColor".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BacklightColor::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BacklightColor {
    pub r#color: u32,
}
impl BacklightColor {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#color: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
