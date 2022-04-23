use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for EquipSlotCategory {
    fn name() -> String {
        "EquipSlotCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EquipSlotCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EquipSlotCategory {
    pub r#main_hand: i8,
    pub r#off_hand: i8,
    pub r#head: i8,
    pub r#body: i8,
    pub r#gloves: i8,
    pub r#waist: i8,
    pub r#legs: i8,
    pub r#feet: i8,
    pub r#ears: i8,
    pub r#neck: i8,
    pub r#wrists: i8,
    pub r#finger_l: i8,
    pub r#finger_r: i8,
    pub r#soul_crystal: i8,
}
impl EquipSlotCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#main_hand: row.field(0usize + offset)?.into_i8()?,
            r#off_hand: row.field(1usize + offset)?.into_i8()?,
            r#head: row.field(2usize + offset)?.into_i8()?,
            r#body: row.field(3usize + offset)?.into_i8()?,
            r#gloves: row.field(4usize + offset)?.into_i8()?,
            r#waist: row.field(5usize + offset)?.into_i8()?,
            r#legs: row.field(6usize + offset)?.into_i8()?,
            r#feet: row.field(7usize + offset)?.into_i8()?,
            r#ears: row.field(8usize + offset)?.into_i8()?,
            r#neck: row.field(9usize + offset)?.into_i8()?,
            r#wrists: row.field(10usize + offset)?.into_i8()?,
            r#finger_l: row.field(11usize + offset)?.into_i8()?,
            r#finger_r: row.field(12usize + offset)?.into_i8()?,
            r#soul_crystal: row.field(13usize + offset)?.into_i8()?,
        })
    }
}
