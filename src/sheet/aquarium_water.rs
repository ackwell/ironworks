use std::result::Result;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for AquariumWater {
    fn name() -> String {
        "AquariumWater".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AquariumWater::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AquariumWater {
    pub r#unknown0: u8,
    pub r#name: SeString,
}
impl AquariumWater {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#name: row.field(1usize + offset)?.into_string()?,
        })
    }
}
