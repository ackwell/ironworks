use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for GcArmyEquipPreset {
    fn name() -> String {
        "GcArmyEquipPreset".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GcArmyEquipPreset::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GcArmyEquipPreset {
    pub r#main_hand: i32,
    pub r#off_hand: i32,
    pub r#head: i32,
    pub r#body: i32,
    pub r#gloves: i32,
    pub r#legs: i32,
    pub r#feet: i32,
}
impl GcArmyEquipPreset {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#main_hand: row.field(0usize + offset)?.into_i32()?,
            r#off_hand: row.field(1usize + offset)?.into_i32()?,
            r#head: row.field(2usize + offset)?.into_i32()?,
            r#body: row.field(3usize + offset)?.into_i32()?,
            r#gloves: row.field(4usize + offset)?.into_i32()?,
            r#legs: row.field(5usize + offset)?.into_i32()?,
            r#feet: row.field(6usize + offset)?.into_i32()?,
        })
    }
}
