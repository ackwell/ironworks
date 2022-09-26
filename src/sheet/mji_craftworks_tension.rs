use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MJICraftworksTension {
    fn name() -> String {
        "MJICraftworksTension".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJICraftworksTension::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJICraftworksTension {}
impl MJICraftworksTension {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
