use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for GatheringType {
    fn name() -> String {
        "GatheringType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringType {
    pub r#name: SeString,
    pub r#icon_main: i32,
    pub r#icon_off: i32,
}
impl GatheringType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#icon_main: row.field(1usize + offset)?.into_i32()?,
            r#icon_off: row.field(2usize + offset)?.into_i32()?,
        })
    }
}
