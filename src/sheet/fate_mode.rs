use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for FateMode {
    fn name() -> String {
        "FateMode".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FateMode::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FateMode {
    pub r#unknown0: u32,
    pub r#motivation_icon: u32,
    pub r#motivation_map_marker: u32,
    pub r#objective_icon: u32,
    pub r#objective_map_marker: u32,
}
impl FateMode {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#motivation_icon: row.field(1usize + offset)?.into_u32()?,
            r#motivation_map_marker: row.field(2usize + offset)?.into_u32()?,
            r#objective_icon: row.field(3usize + offset)?.into_u32()?,
            r#objective_map_marker: row.field(4usize + offset)?.into_u32()?,
        })
    }
}
