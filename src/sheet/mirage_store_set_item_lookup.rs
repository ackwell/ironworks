use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for MirageStoreSetItemLookup {
    fn name() -> String {
        "MirageStoreSetItemLookup".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MirageStoreSetItemLookup::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MirageStoreSetItemLookup {
    pub r#mirage_store_set_item: Vec<u32>,
    pub r#unknown2: u32,
    pub r#unknown3: u32,
    pub r#unknown4: u32,
}
impl MirageStoreSetItemLookup {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#mirage_store_set_item: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
            r#unknown2: row.field(2usize + offset)?.into_u32()?,
            r#unknown3: row.field(3usize + offset)?.into_u32()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
        })
    }
}
