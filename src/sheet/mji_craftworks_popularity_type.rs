use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MJICraftworksPopularityType {
    fn name() -> String {
        "MJICraftworksPopularityType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJICraftworksPopularityType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJICraftworksPopularityType {
    pub r#ratio: u16,
}
impl MJICraftworksPopularityType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#ratio: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
