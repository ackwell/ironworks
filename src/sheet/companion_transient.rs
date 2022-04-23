use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
impl MetadataAdapter for CompanionTransient {
    fn name() -> String {
        "CompanionTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompanionTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompanionTransient {
    pub r#description: SeString,
    pub r#description_enhanced: SeString,
    pub r#tooltip: SeString,
    pub r#special_action_name: SeString,
    pub r#special_action_description: SeString,
    pub r#attack: u8,
    pub r#defense: u8,
    pub r#speed: u8,
    pub r#has_area_attack: bool,
    pub r#strength_gate: bool,
    pub r#strength_eye: bool,
    pub r#strength_shield: bool,
    pub r#strength_arcana: bool,
    pub r#minion_skill_type: u8,
}
impl CompanionTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#description: row.field(0usize + offset)?.into_string()?,
            r#description_enhanced: row.field(1usize + offset)?.into_string()?,
            r#tooltip: row.field(2usize + offset)?.into_string()?,
            r#special_action_name: row.field(3usize + offset)?.into_string()?,
            r#special_action_description: row.field(4usize + offset)?.into_string()?,
            r#attack: row.field(5usize + offset)?.into_u8()?,
            r#defense: row.field(6usize + offset)?.into_u8()?,
            r#speed: row.field(7usize + offset)?.into_u8()?,
            r#has_area_attack: row.field(8usize + offset)?.into_bool()?,
            r#strength_gate: row.field(9usize + offset)?.into_bool()?,
            r#strength_eye: row.field(10usize + offset)?.into_bool()?,
            r#strength_shield: row.field(11usize + offset)?.into_bool()?,
            r#strength_arcana: row.field(12usize + offset)?.into_bool()?,
            r#minion_skill_type: row.field(13usize + offset)?.into_u8()?,
        })
    }
}
