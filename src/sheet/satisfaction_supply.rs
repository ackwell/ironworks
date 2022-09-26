use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for SatisfactionSupply {
    fn name() -> String {
        "SatisfactionSupply".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SatisfactionSupply::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SatisfactionSupply {
    pub r#slot: u8,
    pub r#probability_percent: u8,
    pub r#item: i32,
    pub r#collectability_low: u16,
    pub r#collectability_mid: u16,
    pub r#collectability_high: u16,
    pub r#reward: u16,
}
impl SatisfactionSupply {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#slot: row.field(0usize + offset)?.into_u8()?,
            r#probability_percent: row.field(1usize + offset)?.into_u8()?,
            r#item: row.field(2usize + offset)?.into_i32()?,
            r#collectability_low: row.field(3usize + offset)?.into_u16()?,
            r#collectability_mid: row.field(4usize + offset)?.into_u16()?,
            r#collectability_high: row.field(5usize + offset)?.into_u16()?,
            r#reward: row.field(6usize + offset)?.into_u16()?,
        })
    }
}
