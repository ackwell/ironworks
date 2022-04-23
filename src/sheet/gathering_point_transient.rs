use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for GatheringPointTransient {
    fn name() -> String {
        "GatheringPointTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringPointTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringPointTransient {
    pub r#ephemeral_start_time: u16,
    pub r#ephemeral_end_time: u16,
    pub r#gathering_rare_pop_time_table: i32,
}
impl GatheringPointTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#ephemeral_start_time: row.field(0usize + offset)?.into_u16()?,
            r#ephemeral_end_time: row.field(1usize + offset)?.into_u16()?,
            r#gathering_rare_pop_time_table: row.field(2usize + offset)?.into_i32()?,
        })
    }
}
