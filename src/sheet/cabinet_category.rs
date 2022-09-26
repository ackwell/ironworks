use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for CabinetCategory {
    fn name() -> String {
        "CabinetCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CabinetCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CabinetCategory {
    pub r#menu_order: u8,
    pub r#icon: i32,
    pub r#category: i32,
}
impl CabinetCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#menu_order: row.field(0usize + offset)?.into_u8()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#category: row.field(2usize + offset)?.into_i32()?,
        })
    }
}
