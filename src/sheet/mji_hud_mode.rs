use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
    pub r#unknown3: u32,
}
impl MJIHudMode {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#title: row.field(1usize + offset)?.into_string()?,
            r#icon: row.field(2usize + offset)?.into_u32()?,
            r#unknown3: row.field(3usize + offset)?.into_u32()?,
        })
    }
}
