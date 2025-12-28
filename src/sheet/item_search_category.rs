use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for ItemSearchCategory {
    fn name() -> String {
        "ItemSearchCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemSearchCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemSearchCategory {
    pub r#name: SeString,
    pub r#icon: i32,
    pub r#category: u8,
    pub r#order: u8,
    pub r#class_job: i8,
    pub r#unknown5: bool,
}
impl ItemSearchCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#category: row.field(2usize + offset)?.into_u8()?,
            r#order: row.field(3usize + offset)?.into_u8()?,
            r#class_job: row.field(4usize + offset)?.into_i8()?,
            r#unknown5: row.field(5usize + offset)?.into_bool()?,
        })
    }
}
