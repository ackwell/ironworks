use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#unknown0: u16,
    pub r#unknown1: u8,
    pub r#member: u32,
    pub r#big_image_old: u32,
    pub r#big_image_new: u32,
    pub r#class: u8,
}
impl DawnQuestMember {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#member: row.field(2usize + offset)?.into_u32()?,
            r#big_image_old: row.field(3usize + offset)?.into_u32()?,
            r#big_image_new: row.field(4usize + offset)?.into_u32()?,
            r#class: row.field(5usize + offset)?.into_u8()?,
        })
    }
}
