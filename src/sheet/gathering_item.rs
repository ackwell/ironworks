use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#quest: bool,
    pub r#unknown4: u16,
    pub r#is_hidden: u32,
    pub r#unknown6: bool,
    pub r#unknown7: u32,
    pub r#unknown8: u8,
    pub r#unknown9: u16,
}
impl GatheringItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#gathering_item_level: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#quest: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#is_hidden: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_bool()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#unknown9: row.field(9usize + offset)?.into_u16()?,
        })
    }
}
