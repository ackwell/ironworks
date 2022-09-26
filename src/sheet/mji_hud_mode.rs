use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
impl MetadataAdapter for MJIHudMode {
    fn name() -> String {
        "MJIHudMode".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIHudMode::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIHudMode {
    pub r#name: SeString,
    pub r#title: SeString,
    pub r#icon: u32,
}
impl MJIHudMode {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#title: row.field(1usize + offset)?.into_string()?,
            r#icon: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
