use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#unknown1: bool,
    pub r#output: u32,
}
impl GuidePage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#key: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#output: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
