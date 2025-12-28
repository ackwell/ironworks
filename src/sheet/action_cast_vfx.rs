use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ActionCastVFX {
    fn name() -> String {
        "ActionCastVFX".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActionCastVFX::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActionCastVFX {
    pub r#vfx: u16,
}
impl ActionCastVFX {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#vfx: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
