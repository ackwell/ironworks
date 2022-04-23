use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for SubmarineRank {
    fn name() -> String {
        "SubmarineRank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SubmarineRank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SubmarineRank {
    pub r#capacity: u16,
    pub r#exp_to_next: u32,
    pub r#surveillance_bonus: u8,
    pub r#retrieval_bonus: u8,
    pub r#speed_bonus: u8,
    pub r#range_bonus: u8,
    pub r#favor_bonus: u8,
}
impl SubmarineRank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#capacity: row.field(0usize + offset)?.into_u16()?,
            r#exp_to_next: row.field(1usize + offset)?.into_u32()?,
            r#surveillance_bonus: row.field(2usize + offset)?.into_u8()?,
            r#retrieval_bonus: row.field(3usize + offset)?.into_u8()?,
            r#speed_bonus: row.field(4usize + offset)?.into_u8()?,
            r#range_bonus: row.field(5usize + offset)?.into_u8()?,
            r#favor_bonus: row.field(6usize + offset)?.into_u8()?,
        })
    }
}
