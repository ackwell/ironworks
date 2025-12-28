use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for CharaCardBase {
    fn name() -> String {
        "CharaCardBase".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CharaCardBase::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CharaCardBase {
    pub r#image: i32,
    pub r#font_color: u8,
    pub r#unknown2: bool,
    pub r#unknown3: bool,
    pub r#unknown4: u8,
    pub r#unlock_condition: u16,
    pub r#unknown6: u16,
    pub r#unknown7: u16,
    pub r#unknown8: u16,
    pub r#sort_key: u8,
    pub r#name: u16,
    pub r#unknown11: SeString,
}
impl CharaCardBase {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#image: row.field(0usize + offset)?.into_i32()?,
            r#font_color: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
            r#unlock_condition: row.field(5usize + offset)?.into_u16()?,
            r#unknown6: row.field(6usize + offset)?.into_u16()?,
            r#unknown7: row.field(7usize + offset)?.into_u16()?,
            r#unknown8: row.field(8usize + offset)?.into_u16()?,
            r#sort_key: row.field(9usize + offset)?.into_u8()?,
            r#name: row.field(10usize + offset)?.into_u16()?,
            r#unknown11: row.field(11usize + offset)?.into_string()?,
        })
    }
}
