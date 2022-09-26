use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for AkatsukiNote {
    fn name() -> String {
        "AkatsukiNote".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AkatsukiNote::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AkatsukiNote {}
impl AkatsukiNote {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
