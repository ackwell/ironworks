use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MobHuntTarget {
    fn name() -> String {
        "MobHuntTarget".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MobHuntTarget::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MobHuntTarget {
    pub r#name: u16,
    pub r#fate: u16,
    pub r#icon: u32,
    pub r#territory_type: u16,
    pub r#place_name: u16,
}
impl MobHuntTarget {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_u16()?,
            r#fate: row.field(1usize + offset)?.into_u16()?,
            r#icon: row.field(2usize + offset)?.into_u32()?,
            r#territory_type: row.field(3usize + offset)?.into_u16()?,
            r#place_name: row.field(4usize + offset)?.into_u16()?,
        })
    }
}
