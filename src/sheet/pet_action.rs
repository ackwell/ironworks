use std::result::Result;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for PetAction {
    fn name() -> String {
        "PetAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PetAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PetAction {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#icon: i32,
    pub r#action: u16,
    pub r#pet: u8,
    pub r#master_order: bool,
    pub r#disable_order: bool,
}
impl PetAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#icon: row.field(2usize + offset)?.into_i32()?,
            r#action: row.field(3usize + offset)?.into_u16()?,
            r#pet: row.field(4usize + offset)?.into_u8()?,
            r#master_order: row.field(5usize + offset)?.into_bool()?,
            r#disable_order: row.field(6usize + offset)?.into_bool()?,
        })
    }
}
