use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for QuestLinkMarkerSet {
    fn name() -> String {
        "QuestLinkMarkerSet".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestLinkMarkerSet::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestLinkMarkerSet {
    pub r#unknown0: u16,
    pub r#unknown1: u32,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#unknown4: u8,
    pub r#unknown5: bool,
    pub r#unknown6: bool,
}
impl QuestLinkMarkerSet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
            r#unknown5: row.field(5usize + offset)?.into_bool()?,
            r#unknown6: row.field(6usize + offset)?.into_bool()?,
        })
    }
}
