use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for McGuffin {
    fn name() -> String {
        "McGuffin".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(McGuffin::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct McGuffin {
    pub r#ui_data: u8,
}
impl McGuffin {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#ui_data: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
