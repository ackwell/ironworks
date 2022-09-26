use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for BannerPreset {
    fn name() -> String {
        "BannerPreset".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BannerPreset::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BannerPreset {}
impl BannerPreset {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
