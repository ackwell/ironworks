use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for MirageStoreSetItem {
    fn name() -> String {
        "MirageStoreSetItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MirageStoreSetItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MirageStoreSetItem {
    pub r#unknown0: u32,
    pub r#unknown1: u32,
    pub r#item: Vec<u32>,
}
impl MirageStoreSetItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
            r#item: read_array(
                offset,
                9usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
