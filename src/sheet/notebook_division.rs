use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
impl MetadataAdapter for NotebookDivision {
    fn name() -> String {
        "NotebookDivision".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(NotebookDivision::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct NotebookDivision {
    pub r#name: SeString,
    pub r#notebook_division_category: u8,
    pub r#craft_opening_level: u8,
    pub r#gathering_opening_level: u8,
    pub r#quest_unlock: u32,
    pub r#unknown5: u8,
    pub r#unknown6: bool,
    pub r#crp_craft: bool,
    pub r#bsm_craft: bool,
    pub r#arm_craft: bool,
    pub r#gsm_craft: bool,
    pub r#ltw_craft: bool,
    pub r#wvr_craft: bool,
    pub r#alc_craft: bool,
    pub r#cul_craft: bool,
}
impl NotebookDivision {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#notebook_division_category: row.field(1usize + offset)?.into_u8()?,
            r#craft_opening_level: row.field(2usize + offset)?.into_u8()?,
            r#gathering_opening_level: row.field(3usize + offset)?.into_u8()?,
            r#quest_unlock: row.field(4usize + offset)?.into_u32()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_bool()?,
            r#crp_craft: row.field(7usize + offset)?.into_bool()?,
            r#bsm_craft: row.field(8usize + offset)?.into_bool()?,
            r#arm_craft: row.field(9usize + offset)?.into_bool()?,
            r#gsm_craft: row.field(10usize + offset)?.into_bool()?,
            r#ltw_craft: row.field(11usize + offset)?.into_bool()?,
            r#wvr_craft: row.field(12usize + offset)?.into_bool()?,
            r#alc_craft: row.field(13usize + offset)?.into_bool()?,
            r#cul_craft: row.field(14usize + offset)?.into_bool()?,
        })
    }
}
