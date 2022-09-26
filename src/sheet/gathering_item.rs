use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for GatheringItem {
    fn name() -> String {
        "GatheringItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringItem {
    pub r#item: i32,
    pub r#gathering_item_level: u16,
    pub r#unknown2: bool,
    pub r#quest: u16,
    pub r#unknown4: u32,
    pub r#is_hidden: bool,
}
impl GatheringItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#gathering_item_level: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#quest: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
            r#is_hidden: row.field(5usize + offset)?.into_bool()?,
        })
    }
}
