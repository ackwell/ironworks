use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for BuddyItem {
    fn name() -> String {
        "BuddyItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BuddyItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BuddyItem {
    pub r#item: u16,
    pub r#use_field: bool,
    pub r#use_training: bool,
    pub r#unknown3: bool,
    pub r#status: u8,
}
impl BuddyItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u16()?,
            r#use_field: row.field(1usize + offset)?.into_bool()?,
            r#use_training: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#status: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
