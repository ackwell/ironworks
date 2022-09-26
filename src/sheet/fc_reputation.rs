use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for FCReputation {
    fn name() -> String {
        "FCReputation".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FCReputation::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FCReputation {
    pub r#points_to_next: u32,
    pub r#required_points: u32,
    pub r#discount_rate: u8,
    pub r#color: u32,
    pub r#name: SeString,
}
impl FCReputation {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#points_to_next: row.field(0usize + offset)?.into_u32()?,
            r#required_points: row.field(1usize + offset)?.into_u32()?,
            r#discount_rate: row.field(2usize + offset)?.into_u8()?,
            r#color: row.field(3usize + offset)?.into_u32()?,
            r#name: row.field(4usize + offset)?.into_string()?,
        })
    }
}
