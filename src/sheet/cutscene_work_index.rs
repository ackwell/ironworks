use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for CutsceneWorkIndex {
    fn name() -> String {
        "CutsceneWorkIndex".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CutsceneWorkIndex::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CutsceneWorkIndex {
    pub r#work_index: u16,
}
impl CutsceneWorkIndex {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#work_index: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
