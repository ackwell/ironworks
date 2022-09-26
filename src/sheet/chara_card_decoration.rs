use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for CharaCardDecoration {
    fn name() -> String {
        "CharaCardDecoration".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CharaCardDecoration::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CharaCardDecoration {
    pub r#category: u8,
    pub r#image: i32,
    pub r#unknown2: u8,
    pub r#unlock_condition: u16,
    pub r#sort_key: u16,
    pub r#name: SeString,
}
impl CharaCardDecoration {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: row.field(0usize + offset)?.into_u8()?,
            r#image: row.field(1usize + offset)?.into_i32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unlock_condition: row.field(3usize + offset)?.into_u16()?,
            r#sort_key: row.field(4usize + offset)?.into_u16()?,
            r#name: row.field(5usize + offset)?.into_string()?,
        })
    }
}
