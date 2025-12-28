use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for TreasureHuntRank {
    fn name() -> String {
        "TreasureHuntRank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TreasureHuntRank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TreasureHuntRank {
    pub r#unknown0: u8,
    pub r#icon: u32,
    pub r#item_name: i32,
    pub r#key_item_name: i32,
    pub r#instance_map: i32,
    pub r#max_party_size: u8,
    pub r#treasure_hunt_texture: u8,
    pub r#unknown7: u16,
    pub r#unknown8: bool,
}
impl TreasureHuntRank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#icon: row.field(1usize + offset)?.into_u32()?,
            r#item_name: row.field(2usize + offset)?.into_i32()?,
            r#key_item_name: row.field(3usize + offset)?.into_i32()?,
            r#instance_map: row.field(4usize + offset)?.into_i32()?,
            r#max_party_size: row.field(5usize + offset)?.into_u8()?,
            r#treasure_hunt_texture: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_u16()?,
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
        })
    }
}
