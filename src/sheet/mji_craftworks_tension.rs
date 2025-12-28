use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
pub struct MJICraftworksTension {
    pub r#unknown0: u8,
}
impl MJICraftworksTension {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
