use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
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
}
impl MJIVillageAppearanceUI {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#floor: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
