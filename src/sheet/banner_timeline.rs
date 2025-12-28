use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for BannerTimeline {
    fn name() -> String {
        "BannerTimeline".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BannerTimeline::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BannerTimeline {
    pub r#type: u8,
    pub r#additional_data: u32,
    pub r#accept_class_job_category: u8,
    pub r#category: u8,
    pub r#unlock_condition: u16,
    pub r#unknown5: u16,
    pub r#unknown6: u16,
    pub r#sort_key: u16,
    pub r#icon: u16,
    pub r#name: i32,
    pub r#unknown10: SeString,
}
impl BannerTimeline {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#additional_data: row.field(1usize + offset)?.into_u32()?,
            r#accept_class_job_category: row.field(2usize + offset)?.into_u8()?,
            r#category: row.field(3usize + offset)?.into_u8()?,
            r#unlock_condition: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u16()?,
            r#unknown6: row.field(6usize + offset)?.into_u16()?,
            r#sort_key: row.field(7usize + offset)?.into_u16()?,
            r#icon: row.field(8usize + offset)?.into_u16()?,
            r#name: row.field(9usize + offset)?.into_i32()?,
            r#unknown10: row.field(10usize + offset)?.into_string()?,
        })
    }
}
