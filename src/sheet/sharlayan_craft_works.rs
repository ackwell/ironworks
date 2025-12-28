use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for SharlayanCraftWorks {
    fn name() -> String {
        "SharlayanCraftWorks".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SharlayanCraftWorks::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SharlayanCraftWorks {
    pub r#unknown0: u32,
    pub r#unknown1: u16,
    pub r#unknown2: SeString,
}
impl SharlayanCraftWorks {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_string()?,
        })
    }
}
