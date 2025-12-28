use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for Resident {
    fn name() -> String {
        "Resident".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Resident::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Resident {
    pub r#unknown0: u8,
    pub r#model: u64,
    pub r#npc_yell: i32,
    pub r#unknown3: u16,
    pub r#resident_motion_type: u8,
}
impl Resident {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#model: row.field(1usize + offset)?.into_u64()?,
            r#npc_yell: row.field(2usize + offset)?.into_i32()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#resident_motion_type: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
