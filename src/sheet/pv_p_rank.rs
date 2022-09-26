use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for PvPRank {
    fn name() -> String {
        "PvPRank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PvPRank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PvPRank {
    pub r#exp_required: u32,
}
impl PvPRank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#exp_required: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
