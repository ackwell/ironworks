use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for BannerFrame {
    fn name() -> String {
        "BannerFrame".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BannerFrame::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BannerFrame {
    pub r#image: i32,
    pub r#icon: i32,
    pub r#unknown2: u8,
    pub r#unlock_condition: u16,
    pub r#unknown4: u16,
    pub r#unknown5: u16,
    pub r#sort_key: u16,
    pub r#name: u16,
    pub r#unknown8: SeString,
}
impl BannerFrame {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#image: row.field(0usize + offset)?.into_i32()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unlock_condition: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u16()?,
            r#sort_key: row.field(6usize + offset)?.into_u16()?,
            r#name: row.field(7usize + offset)?.into_u16()?,
            r#unknown8: row.field(8usize + offset)?.into_string()?,
        })
    }
}
