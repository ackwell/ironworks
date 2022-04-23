use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for ZoneSharedGroup {
    fn name() -> String {
        "ZoneSharedGroup".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ZoneSharedGroup::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ZoneSharedGroup {
    pub r#lgb_shared_group: u32,
    pub r#unknown1: u8,
    pub r#quest0: u32,
    pub r#seq0: u32,
    pub r#unknown4: bool,
    pub r#unknown5: u8,
    pub r#quest1: u32,
    pub r#seq1: u32,
    pub r#unknown8: bool,
    pub r#unknown9: u8,
    pub r#quest2: u32,
    pub r#seq2: u32,
    pub r#unknown12: bool,
    pub r#unknown13: u8,
    pub r#quest3: u32,
    pub r#seq3: u32,
    pub r#unknown16: bool,
    pub r#unknown17: u8,
    pub r#quest4: u32,
    pub r#seq4: u32,
    pub r#unknown20: bool,
    pub r#unknown21: u8,
    pub r#quest5: u32,
    pub r#seq5: u32,
}
impl ZoneSharedGroup {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#lgb_shared_group: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#quest0: row.field(2usize + offset)?.into_u32()?,
            r#seq0: row.field(3usize + offset)?.into_u32()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#quest1: row.field(6usize + offset)?.into_u32()?,
            r#seq1: row.field(7usize + offset)?.into_u32()?,
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#quest2: row.field(10usize + offset)?.into_u32()?,
            r#seq2: row.field(11usize + offset)?.into_u32()?,
            r#unknown12: row.field(12usize + offset)?.into_bool()?,
            r#unknown13: row.field(13usize + offset)?.into_u8()?,
            r#quest3: row.field(14usize + offset)?.into_u32()?,
            r#seq3: row.field(15usize + offset)?.into_u32()?,
            r#unknown16: row.field(16usize + offset)?.into_bool()?,
            r#unknown17: row.field(17usize + offset)?.into_u8()?,
            r#quest4: row.field(18usize + offset)?.into_u32()?,
            r#seq4: row.field(19usize + offset)?.into_u32()?,
            r#unknown20: row.field(20usize + offset)?.into_bool()?,
            r#unknown21: row.field(21usize + offset)?.into_u8()?,
            r#quest5: row.field(22usize + offset)?.into_u32()?,
            r#seq5: row.field(23usize + offset)?.into_u32()?,
        })
    }
}
