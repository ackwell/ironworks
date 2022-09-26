use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for BannerObtainHintType {
    fn name() -> String {
        "BannerObtainHintType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BannerObtainHintType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BannerObtainHintType {}
impl BannerObtainHintType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
