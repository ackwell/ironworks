use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for HousingAethernet {
    fn name() -> String {
        "HousingAethernet".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HousingAethernet::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HousingAethernet {
    pub r#level: u32,
    pub r#territory_type: u16,
    pub r#place_name: u16,
    pub r#order: u8,
}
impl HousingAethernet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#level: row.field(0usize + offset)?.into_u32()?,
            r#territory_type: row.field(1usize + offset)?.into_u16()?,
            r#place_name: row.field(2usize + offset)?.into_u16()?,
            r#order: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
