use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for IKDFishParam {
    fn name() -> String {
        "IKDFishParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(IKDFishParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct IKDFishParam {
    pub r#fish: u32,
    pub r#ikd_content_bonus: u8,
    pub r#unknown2: u8,
}
impl IKDFishParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#fish: row.field(0usize + offset)?.into_u32()?,
            r#ikd_content_bonus: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
