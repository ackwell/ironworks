use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for CharaMakeCustomize {
    fn name() -> String {
        "CharaMakeCustomize".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CharaMakeCustomize::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CharaMakeCustomize {
    pub r#feature_id: u8,
    pub r#icon: u32,
    pub r#data: u16,
    pub r#is_purchasable: bool,
    pub r#hint: u32,
    pub r#hint_item: u32,
    pub r#unknown6: u8,
}
impl CharaMakeCustomize {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#feature_id: row.field(0usize + offset)?.into_u8()?,
            r#icon: row.field(1usize + offset)?.into_u32()?,
            r#data: row.field(2usize + offset)?.into_u16()?,
            r#is_purchasable: row.field(3usize + offset)?.into_bool()?,
            r#hint: row.field(4usize + offset)?.into_u32()?,
            r#hint_item: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
        })
    }
}
