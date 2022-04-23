use crate::error::PopulateError;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for World {
    fn name() -> String {
        "World".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(World::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct World {
    pub r#internal_name: SeString,
    pub r#name: SeString,
    pub r#region: u8,
    pub r#user_type: u8,
    pub r#data_center: u8,
    pub r#is_public: bool,
}
impl World {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#internal_name: row.field(0usize + offset)?.into_string()?,
            r#name: row.field(1usize + offset)?.into_string()?,
            r#region: row.field(2usize + offset)?.into_u8()?,
            r#user_type: row.field(3usize + offset)?.into_u8()?,
            r#data_center: row.field(4usize + offset)?.into_u8()?,
            r#is_public: row.field(5usize + offset)?.into_bool()?,
        })
    }
}
