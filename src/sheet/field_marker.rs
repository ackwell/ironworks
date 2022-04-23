use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for FieldMarker {
    fn name() -> String {
        "FieldMarker".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FieldMarker::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FieldMarker {
    pub r#vfx: i32,
    pub r#ui_icon: u16,
    pub r#map_icon: u16,
    pub r#name: SeString,
}
impl FieldMarker {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#vfx: row.field(0usize + offset)?.into_i32()?,
            r#ui_icon: row.field(1usize + offset)?.into_u16()?,
            r#map_icon: row.field(2usize + offset)?.into_u16()?,
            r#name: row.field(3usize + offset)?.into_string()?,
        })
    }
}
