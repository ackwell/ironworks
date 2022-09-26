use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
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
}
impl MJIRecipeMaterial {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item_pouch: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
