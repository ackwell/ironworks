use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
    pub r#unknown0: bool,
    pub r#list: bool,
    pub r#unknown2: bool,
    pub r#priority: u8,
    pub r#icon: u32,
    pub r#unknown5: i32,
    pub r#name: SeString,
}
impl OnlineStatus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#list: row.field(1usize + offset)?.into_bool()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#priority: row.field(3usize + offset)?.into_u8()?,
            r#icon: row.field(4usize + offset)?.into_u32()?,
            r#unknown5: row.field(5usize + offset)?.into_i32()?,
            r#name: row.field(6usize + offset)?.into_string()?,
        })
    }
}
