use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ActionCastTimeline {
    fn name() -> String {
        "ActionCastTimeline".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActionCastTimeline::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActionCastTimeline {
    pub r#name: u16,
    pub r#vfx: u16,
}
impl ActionCastTimeline {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_u16()?,
            r#vfx: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
