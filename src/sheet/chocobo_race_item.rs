use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for ChocoboRaceItem {
    fn name() -> String {
        "ChocoboRaceItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ChocoboRaceItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ChocoboRaceItem {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#icon: u32,
}
impl ChocoboRaceItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#icon: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
