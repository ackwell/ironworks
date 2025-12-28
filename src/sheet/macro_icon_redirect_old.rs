use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MacroIconRedirectOld {
    fn name() -> String {
        "MacroIconRedirectOld".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MacroIconRedirectOld::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MacroIconRedirectOld {
    pub r#icon_old: u32,
    pub r#icon_new: i32,
}
impl MacroIconRedirectOld {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon_old: row.field(0usize + offset)?.into_u32()?,
            r#icon_new: row.field(1usize + offset)?.into_i32()?,
        })
    }
}
