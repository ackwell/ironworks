use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for GatheringItemLevelConvertTable {
    fn name() -> String {
        "GatheringItemLevelConvertTable".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringItemLevelConvertTable::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringItemLevelConvertTable {
    pub r#gathering_item_level: u8,
    pub r#stars: u8,
}
impl GatheringItemLevelConvertTable {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#gathering_item_level: row.field(0usize + offset)?.into_u8()?,
            r#stars: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
