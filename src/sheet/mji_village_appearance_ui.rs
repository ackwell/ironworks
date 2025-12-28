use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MJIVillageAppearanceUI {
    fn name() -> String {
        "MJIVillageAppearanceUI".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIVillageAppearanceUI::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIVillageAppearanceUI {
    pub r#floor: i32,
    pub r#unknown1: u16,
    pub r#unknown2: u16,
}
impl MJIVillageAppearanceUI {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#floor: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
