use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for AquariumFish {
    fn name() -> String {
        "AquariumFish".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AquariumFish::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AquariumFish {
    pub r#aquarium_water: u8,
    pub r#size: u8,
    pub r#item: u32,
}
impl AquariumFish {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#aquarium_water: row.field(0usize + offset)?.into_u8()?,
            r#size: row.field(1usize + offset)?.into_u8()?,
            r#item: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
