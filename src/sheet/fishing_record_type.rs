use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for FishingRecordType {
    fn name() -> String {
        "FishingRecordType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FishingRecordType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FishingRecordType {
    pub r#addon: i32,
    pub r#rank_b_requirement: u16,
    pub r#rank_a_requirement: u16,
    pub r#rank_aa_requirement: u16,
    pub r#rank_aaa_requirement: u16,
    pub r#rank_s_requirement: u16,
    pub r#is_spearfishing: u8,
}
impl FishingRecordType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#addon: row.field(0usize + offset)?.into_i32()?,
            r#rank_b_requirement: row.field(1usize + offset)?.into_u16()?,
            r#rank_a_requirement: row.field(2usize + offset)?.into_u16()?,
            r#rank_aa_requirement: row.field(3usize + offset)?.into_u16()?,
            r#rank_aaa_requirement: row.field(4usize + offset)?.into_u16()?,
            r#rank_s_requirement: row.field(5usize + offset)?.into_u16()?,
            r#is_spearfishing: row.field(6usize + offset)?.into_u8()?,
        })
    }
}
