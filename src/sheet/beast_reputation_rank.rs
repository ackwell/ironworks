use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for BeastReputationRank {
    fn name() -> String {
        "BeastReputationRank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BeastReputationRank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BeastReputationRank {
    pub r#required_reputation: u16,
    pub r#name: SeString,
    pub r#allied_names: SeString,
    pub r#color: u32,
}
impl BeastReputationRank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#required_reputation: row.field(0usize + offset)?.into_u16()?,
            r#name: row.field(1usize + offset)?.into_string()?,
            r#allied_names: row.field(2usize + offset)?.into_string()?,
            r#color: row.field(3usize + offset)?.into_u32()?,
        })
    }
}
