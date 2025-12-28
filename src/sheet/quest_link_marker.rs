use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for QuestLinkMarker {
    fn name() -> String {
        "QuestLinkMarker".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestLinkMarker::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestLinkMarker {
    pub r#source_map: u32,
    pub r#level: u32,
    pub r#target_map: u32,
    pub r#unknown3: u8,
    pub r#unknown4: bool,
}
impl QuestLinkMarker {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#source_map: row.field(0usize + offset)?.into_u32()?,
            r#level: row.field(1usize + offset)?.into_u32()?,
            r#target_map: row.field(2usize + offset)?.into_u32()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
        })
    }
}
