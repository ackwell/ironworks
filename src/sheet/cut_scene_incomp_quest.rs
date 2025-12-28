use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for CutSceneIncompQuest {
    fn name() -> String {
        "CutSceneIncompQuest".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CutSceneIncompQuest::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CutSceneIncompQuest {
    pub r#quest: u32,
}
impl CutSceneIncompQuest {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
