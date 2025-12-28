use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for BuddyRank {
    fn name() -> String {
        "BuddyRank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BuddyRank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BuddyRank {
    pub r#exp_required: u32,
}
impl BuddyRank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#exp_required: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
