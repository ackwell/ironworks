use std::convert::Infallible;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for DawnQuestMember {
    fn name() -> String {
        "DawnQuestMember".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DawnQuestMember::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DawnQuestMember {
    pub r#member: u32,
    pub r#image_name: u32,
    pub r#big_image_old: u32,
    pub r#big_image_new: u8,
    pub r#class: Option<Infallible>,
}
impl DawnQuestMember {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#member: row.field(0usize + offset)?.into_u32()?,
            r#image_name: row.field(1usize + offset)?.into_u32()?,
            r#big_image_old: row.field(2usize + offset)?.into_u32()?,
            r#big_image_new: row.field(3usize + offset)?.into_u8()?,
            r#class: None,
        })
    }
}
