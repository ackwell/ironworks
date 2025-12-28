use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for CharaCardHeader {
    fn name() -> String {
        "CharaCardHeader".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CharaCardHeader::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CharaCardHeader {
    pub r#top_image: i32,
    pub r#bottom_image: i32,
    pub r#font_color: u8,
    pub r#unknown3: u8,
    pub r#unknown4: u8,
    pub r#unknown5: u8,
    pub r#unlock_condition: u16,
    pub r#unknown7: u16,
    pub r#unknown8: u16,
    pub r#unknown9: u16,
    pub r#sort_key: u8,
    pub r#name: u16,
    pub r#unknown12: SeString,
}
impl CharaCardHeader {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#top_image: row.field(0usize + offset)?.into_i32()?,
            r#bottom_image: row.field(1usize + offset)?.into_i32()?,
            r#font_color: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unlock_condition: row.field(6usize + offset)?.into_u16()?,
            r#unknown7: row.field(7usize + offset)?.into_u16()?,
            r#unknown8: row.field(8usize + offset)?.into_u16()?,
            r#unknown9: row.field(9usize + offset)?.into_u16()?,
            r#sort_key: row.field(10usize + offset)?.into_u8()?,
            r#name: row.field(11usize + offset)?.into_u16()?,
            r#unknown12: row.field(12usize + offset)?.into_string()?,
        })
    }
}
