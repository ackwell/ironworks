use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
    pub r#sub_type: u8,
    pub r#image: i32,
    pub r#unknown3: u8,
    pub r#unlock_condition: u16,
    pub r#unknown5: u16,
    pub r#unknown6: u16,
    pub r#unknown7: u16,
    pub r#sort_key: u8,
    pub r#name: u16,
    pub r#unknown10: SeString,
}
impl CharaCardDecoration {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: row.field(0usize + offset)?.into_u8()?,
            r#sub_type: row.field(1usize + offset)?.into_u8()?,
            r#image: row.field(2usize + offset)?.into_i32()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unlock_condition: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u16()?,
            r#unknown6: row.field(6usize + offset)?.into_u16()?,
            r#unknown7: row.field(7usize + offset)?.into_u16()?,
            r#sort_key: row.field(8usize + offset)?.into_u8()?,
            r#name: row.field(9usize + offset)?.into_u16()?,
            r#unknown10: row.field(10usize + offset)?.into_string()?,
        })
    }
}
