use std::result::Result;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
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
    pub r#sort_key: u16,
    pub r#icon: i32,
    pub r#name: SeString,
}
impl BannerTimeline {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#additional_data: row.field(1usize + offset)?.into_u32()?,
            r#accept_class_job_category: row.field(2usize + offset)?.into_u8()?,
            r#category: row.field(3usize + offset)?.into_u8()?,
            r#unlock_condition: row.field(4usize + offset)?.into_u16()?,
            r#sort_key: row.field(5usize + offset)?.into_u16()?,
            r#icon: row.field(6usize + offset)?.into_i32()?,
            r#name: row.field(7usize + offset)?.into_string()?,
        })
    }
}
