use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for CraftType {
    fn name() -> String {
        "CraftType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CraftType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CraftType {
    pub r#main_physical: u8,
    pub r#sub_physical: u8,
    pub r#name: SeString,
}
impl CraftType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#main_physical: row.field(0usize + offset)?.into_u8()?,
            r#sub_physical: row.field(1usize + offset)?.into_u8()?,
            r#name: row.field(2usize + offset)?.into_string()?,
        })
    }
}
