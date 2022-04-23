use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
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
}
impl MacroIcon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
