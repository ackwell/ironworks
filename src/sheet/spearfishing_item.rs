use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for SpearfishingItem {
    fn name() -> String {
        "SpearfishingItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SpearfishingItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SpearfishingItem {
    pub r#description: SeString,
    pub r#item: i32,
    pub r#gathering_item_level: u16,
    pub r#fishing_record_type: bool,
    pub r#territory_type: bool,
    pub r#is_visible: u8,
}
impl SpearfishingItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#description: row.field(0usize + offset)?.into_string()?,
            r#item: row.field(1usize + offset)?.into_i32()?,
            r#gathering_item_level: row.field(2usize + offset)?.into_u16()?,
            r#fishing_record_type: row.field(3usize + offset)?.into_bool()?,
            r#territory_type: row.field(4usize + offset)?.into_bool()?,
            r#is_visible: row.field(5usize + offset)?.into_u8()?,
        })
    }
}
