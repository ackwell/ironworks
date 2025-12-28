use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MJIRecipeMaterial {
    fn name() -> String {
        "MJIRecipeMaterial".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIRecipeMaterial::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIRecipeMaterial {
    pub r#item_pouch: i32,
    pub r#unknown1: i32,
}
impl MJIRecipeMaterial {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item_pouch: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_i32()?,
        })
    }
}
