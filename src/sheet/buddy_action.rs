use std::result::Result;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for BuddyAction {
    fn name() -> String {
        "BuddyAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BuddyAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BuddyAction {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#icon: i32,
    pub r#icon_status: i32,
    pub r#reward: u16,
    pub r#sort: u8,
}
impl BuddyAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#icon: row.field(2usize + offset)?.into_i32()?,
            r#icon_status: row.field(3usize + offset)?.into_i32()?,
            r#reward: row.field(4usize + offset)?.into_u16()?,
            r#sort: row.field(5usize + offset)?.into_u8()?,
        })
    }
}
