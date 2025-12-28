use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for GatheringPointBonus {
    fn name() -> String {
        "GatheringPointBonus".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringPointBonus::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringPointBonus {
    pub r#condition: u8,
    pub r#condition_value: u32,
    pub r#unknown2: u16,
    pub r#bonus_type: u8,
    pub r#bonus_value: u16,
    pub r#unknown5: u16,
    pub r#unknown6: bool,
    pub r#unknown7: u32,
}
impl GatheringPointBonus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#condition: row.field(0usize + offset)?.into_u8()?,
            r#condition_value: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#bonus_type: row.field(3usize + offset)?.into_u8()?,
            r#bonus_value: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u16()?,
            r#unknown6: row.field(6usize + offset)?.into_bool()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
        })
    }
}
