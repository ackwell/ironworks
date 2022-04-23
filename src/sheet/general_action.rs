use std::result::Result;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for GeneralAction {
    fn name() -> String {
        "GeneralAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GeneralAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GeneralAction {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#unknown2: u8,
    pub r#action: u16,
    pub r#unlock_link: u16,
    pub r#recast: u8,
    pub r#ui_priority: u8,
    pub r#icon: i32,
}
impl GeneralAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#action: row.field(3usize + offset)?.into_u16()?,
            r#unlock_link: row.field(4usize + offset)?.into_u16()?,
            r#recast: row.field(5usize + offset)?.into_u8()?,
            r#ui_priority: row.field(6usize + offset)?.into_u8()?,
            r#icon: row.field(7usize + offset)?.into_i32()?,
        })
    }
}
