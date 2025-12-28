use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for DeepDungeonLayer {
    fn name() -> String {
        "DeepDungeonLayer".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DeepDungeonLayer::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DeepDungeonLayer {
    pub r#deep_dungeon: u8,
    pub r#floor_set: u8,
    pub r#room_a: u16,
    pub r#room_b: u16,
    pub r#room_c: u16,
    pub r#wep_min_lv: u8,
    pub r#armour_min_lv: u8,
    pub r#unknown7: bool,
}
impl DeepDungeonLayer {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#deep_dungeon: row.field(0usize + offset)?.into_u8()?,
            r#floor_set: row.field(1usize + offset)?.into_u8()?,
            r#room_a: row.field(2usize + offset)?.into_u16()?,
            r#room_b: row.field(3usize + offset)?.into_u16()?,
            r#room_c: row.field(4usize + offset)?.into_u16()?,
            r#wep_min_lv: row.field(5usize + offset)?.into_u8()?,
            r#armour_min_lv: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_bool()?,
        })
    }
}
