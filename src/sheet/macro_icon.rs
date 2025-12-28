use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MacroIcon {
    fn name() -> String {
        "MacroIcon".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MacroIcon::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MacroIcon {
    pub r#icon: i32,
    pub r#unknown1: u16,
}
impl MacroIcon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
