use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for FishingRecordTypeTransient {
    fn name() -> String {
        "FishingRecordTypeTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FishingRecordTypeTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FishingRecordTypeTransient {
    pub r#image: i32,
}
impl FishingRecordTypeTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#image: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
