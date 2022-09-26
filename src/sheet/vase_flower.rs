use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for VaseFlower {
    fn name() -> String {
        "VaseFlower".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(VaseFlower::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct VaseFlower {
    pub r#unknown0: u16,
    pub r#unknown1: u8,
    pub r#unknown2: u8,
    pub r#item: u32,
}
impl VaseFlower {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#item: row.field(3usize + offset)?.into_u32()?,
        })
    }
}
