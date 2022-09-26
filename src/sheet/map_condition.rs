use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MapCondition {
    fn name() -> String {
        "MapCondition".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MapCondition::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MapCondition {
    pub r#unknown0: u16,
    pub r#quest: i32,
}
impl MapCondition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
            r#quest: row.field(1usize + offset)?.into_i32()?,
        })
    }
}
