use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for GuidePage {
    fn name() -> String {
        "GuidePage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GuidePage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GuidePage {
    pub r#key: u8,
    pub r#output: u32,
}
impl GuidePage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#key: row.field(0usize + offset)?.into_u8()?,
            r#output: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
