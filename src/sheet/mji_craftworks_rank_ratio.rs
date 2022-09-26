use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for MJICraftworksRankRatio {
    fn name() -> String {
        "MJICraftworksRankRatio".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJICraftworksRankRatio::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJICraftworksRankRatio {
    pub r#ratio: u16,
}
impl MJICraftworksRankRatio {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#ratio: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
