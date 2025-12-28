use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for GardeningSeed {
    fn name() -> String {
        "GardeningSeed".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GardeningSeed::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GardeningSeed {
    pub r#item: u32,
    pub r#model_id: u16,
    pub r#icon: u32,
    pub r#se: bool,
    pub r#unknown4: bool,
    pub r#unknown5: u8,
}
impl GardeningSeed {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u32()?,
            r#model_id: row.field(1usize + offset)?.into_u16()?,
            r#icon: row.field(2usize + offset)?.into_u32()?,
            r#se: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
        })
    }
}
