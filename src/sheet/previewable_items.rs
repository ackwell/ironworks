use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for PreviewableItems {
    fn name() -> String {
        "PreviewableItems".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PreviewableItems::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PreviewableItems {
    pub r#unknown0: u16,
}
impl PreviewableItems {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
