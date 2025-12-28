use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
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
    pub r#unknown3: u16,
    pub r#unknown4: bool,
    pub r#fishing_record_type: u8,
    pub r#territory_type: u16,
    pub r#unknown7: u16,
    pub r#is_visible: bool,
}
impl SpearfishingItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#description: row.field(0usize + offset)?.into_string()?,
            r#item: row.field(1usize + offset)?.into_i32()?,
            r#gathering_item_level: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#fishing_record_type: row.field(5usize + offset)?.into_u8()?,
            r#territory_type: row.field(6usize + offset)?.into_u16()?,
            r#unknown7: row.field(7usize + offset)?.into_u16()?,
            r#is_visible: row.field(8usize + offset)?.into_bool()?,
        })
    }
}
