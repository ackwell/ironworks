use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
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
    pub r#unlock_condition: u16,
    pub r#sort_key: u16,
    pub r#name: SeString,
}
impl BannerFrame {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#image: row.field(0usize + offset)?.into_i32()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#unlock_condition: row.field(2usize + offset)?.into_u16()?,
            r#sort_key: row.field(3usize + offset)?.into_u16()?,
            r#name: row.field(4usize + offset)?.into_string()?,
        })
    }
}
