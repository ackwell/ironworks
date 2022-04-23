use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for EurekaAethernet {
    fn name() -> String {
        "EurekaAethernet".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EurekaAethernet::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EurekaAethernet {
    pub r#location: u16,
}
impl EurekaAethernet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#location: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
