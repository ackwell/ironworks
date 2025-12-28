use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for EurekaMagiciteItem {
    fn name() -> String {
        "EurekaMagiciteItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EurekaMagiciteItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EurekaMagiciteItem {
    pub r#eureka_magicite_item_type: u8,
    pub r#class_job_category: u8,
    pub r#item: u32,
}
impl EurekaMagiciteItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#eureka_magicite_item_type: row.field(0usize + offset)?.into_u8()?,
            r#class_job_category: row.field(1usize + offset)?.into_u8()?,
            r#item: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
