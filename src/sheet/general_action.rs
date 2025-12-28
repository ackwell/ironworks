use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
    pub r#unknown8: bool,
    pub r#unknown9: bool,
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
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
            r#unknown9: row.field(9usize + offset)?.into_bool()?,
        })
    }
}
