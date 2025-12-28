use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for DeepDungeonFloorEffectUI {
    fn name() -> String {
        "DeepDungeonFloorEffectUI".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DeepDungeonFloorEffectUI::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DeepDungeonFloorEffectUI {
    pub r#icon: u32,
    pub r#name: SeString,
    pub r#description: SeString,
}
impl DeepDungeonFloorEffectUI {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_u32()?,
            r#name: row.field(1usize + offset)?.into_string()?,
            r#description: row.field(2usize + offset)?.into_string()?,
        })
    }
}
