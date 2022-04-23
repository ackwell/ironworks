use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for FateTokenType {
    fn name() -> String {
        "FateTokenType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FateTokenType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FateTokenType {
    pub r#currency: u32,
}
impl FateTokenType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#currency: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
