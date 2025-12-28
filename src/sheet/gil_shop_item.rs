use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for GilShopItem {
    fn name() -> String {
        "GilShopItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GilShopItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GilShopItem {
    pub r#item: i32,
    pub r#unknown1: bool,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#quest_required: Vec<i32>,
    pub r#achievement_required: i32,
    pub r#unknown7: u8,
    pub r#state_required: u16,
    pub r#patch: u16,
}
impl GilShopItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#quest_required: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(4usize + offset)?.into_i32()?) },
            )?,
            r#achievement_required: row.field(6usize + offset)?.into_i32()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
            r#state_required: row.field(8usize + offset)?.into_u16()?,
            r#patch: row.field(9usize + offset)?.into_u16()?,
        })
    }
}
