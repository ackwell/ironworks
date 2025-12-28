use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for QuestLinkMarkerIcon {
    fn name() -> String {
        "QuestLinkMarkerIcon".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestLinkMarkerIcon::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestLinkMarkerIcon {
    pub r#icon: u32,
}
impl QuestLinkMarkerIcon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
