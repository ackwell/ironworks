use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for AetheryteTransient {
    fn name() -> String {
        "AetheryteTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AetheryteTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AetheryteTransient {
    pub r#unknown0: bool,
}
impl AetheryteTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
        })
    }
}
