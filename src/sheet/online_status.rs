use std::result::Result;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for OnlineStatus {
    fn name() -> String {
        "OnlineStatus".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(OnlineStatus::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct OnlineStatus {
    pub r#list: bool,
    pub r#unknown1: bool,
    pub r#priority: u8,
    pub r#name: SeString,
    pub r#icon: u32,
}
impl OnlineStatus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#list: row.field(0usize + offset)?.into_bool()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#priority: row.field(2usize + offset)?.into_u8()?,
            r#name: row.field(3usize + offset)?.into_string()?,
            r#icon: row.field(4usize + offset)?.into_u32()?,
        })
    }
}
