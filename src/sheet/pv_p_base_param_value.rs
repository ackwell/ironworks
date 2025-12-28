use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for PvPBaseParamValue {
    fn name() -> String {
        "PvPBaseParamValue".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PvPBaseParamValue::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PvPBaseParamValue {
    pub r#unknown0: u32,
    pub r#unknown1: u32,
    pub r#unknown2: u32,
}
impl PvPBaseParamValue {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
